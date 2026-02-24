mod discover;
mod features;
mod gen_aws_json;
mod gen_aws_query;
mod gen_rest_json;
mod gen_rpcv2_cbor;
mod gen_serializers;
mod merge;
mod model;

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use model::{Protocol, ServiceModel};

#[derive(Parser)]
#[command(
    name = "smithy-codegen",
    about = "Generate winterbaume handler scaffolding from Smithy 2.0 JSON API models"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Path to the vendor/api-models-aws/models directory.
    #[arg(long, default_value = "vendor/api-models-aws/models", global = true)]
    models_dir: PathBuf,

    /// Path to the crates directory.
    #[arg(long, default_value = "crates", global = true)]
    crates_dir: PathBuf,
}

#[derive(Subcommand)]
enum Command {
    /// Show info about a service's Smithy model (protocol, operation count, etc.)
    Info {
        /// Service name (e.g., "kms", "ses", "iam")
        service: String,
    },

    /// List all operations for a service and their implementation status.
    Ops {
        /// Service name (e.g., "kms", "ses", "iam")
        service: String,
    },

    /// Inject stub match arms for unimplemented operations into an existing handlers.rs.
    Inject {
        /// Service name (e.g., "kms", "ses", "iam")
        service: String,

        /// Dry run: print what would be generated without modifying files.
        #[arg(long)]
        dry_run: bool,
    },

    /// Generate a complete handlers.rs scaffold (for new services).
    Scaffold {
        /// Service name (e.g., "kms", "ses", "iam")
        service: String,

        /// Name of the service struct (e.g., "KmsService")
        #[arg(long)]
        service_struct: Option<String>,

        /// Name of the state type (e.g., "KmsState")
        #[arg(long)]
        state_type: Option<String>,
    },

    /// List all known service mappings.
    ListServices,

    /// Inject stubs for all services at once.
    InjectAll {
        /// Dry run: print what would be generated without modifying files.
        #[arg(long)]
        dry_run: bool,
    },

    /// Generate typed response serializers (wire.rs) from Smithy output shapes.
    GenSerializers {
        /// Service name (e.g., "kms", "ses", "iam")
        service: String,

        /// Output file path for wire.rs (serializer functions). Defaults to stdout.
        #[arg(long, short)]
        output: Option<PathBuf>,

        /// Combine structs and serializers into a single wire.rs file instead of
        /// splitting into model.rs (structs) + wire.rs (serializers).
        #[arg(long)]
        combined: bool,

        /// Output file path for model.rs (struct definitions).
        /// Defaults to model.rs in the same directory as the wire.rs output.
        #[arg(long)]
        model_output: Option<PathBuf>,

        /// Path to a TOML mapping operation names to Cargo feature slugs.
        /// When provided, generated code is gated with `#[cfg(feature = ...)]`
        /// per operation; types referenced by multiple feature groups are
        /// emitted unconditionally as shared core code.
        #[arg(long)]
        features_map: Option<PathBuf>,
    },
}

/// Run rustfmt on a generated file so that it passes `cargo fmt -- --check`.
/// Silently ignores failures (e.g., rustfmt not in PATH).
fn rustfmt_file(path: &Path) {
    let _ = std::process::Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .arg(path)
        .status();
}

/// Ensure the generated code ends with exactly one newline.
fn normalize_trailing_newline(mut s: String) -> String {
    let trimmed = s.trim_end_matches('\n');
    let len = trimmed.len();
    s.truncate(len);
    s.push('\n');
    s
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Command::Info { service } => cmd_info(&service, &cli.models_dir),
        Command::Ops { service } => cmd_ops(&service, &cli.models_dir, &cli.crates_dir),
        Command::Inject { service, dry_run } => {
            cmd_inject(&service, &cli.models_dir, &cli.crates_dir, dry_run)
        }
        Command::Scaffold {
            service,
            service_struct,
            state_type,
        } => cmd_scaffold(&service, &cli.models_dir, service_struct, state_type),
        Command::ListServices => cmd_list_services(),
        Command::InjectAll { dry_run } => cmd_inject_all(&cli.models_dir, &cli.crates_dir, dry_run),
        Command::GenSerializers {
            service,
            output,
            combined,
            model_output,
            features_map,
        } => cmd_gen_serializers(
            &service,
            &cli.models_dir,
            output,
            !combined,
            model_output,
            features_map,
        ),
    }
}

fn load_model(service: &str, models_dir: &Path) -> Result<ServiceModel> {
    let model_path = discover::find_model_file(service, models_dir)?;
    ServiceModel::from_file(&model_path)
}

fn cmd_info(service: &str, models_dir: &Path) -> Result<()> {
    let model = load_model(service, models_dir)?;

    println!("Service:     {}", model.service_name);
    println!("SDK ID:      {}", model.sdk_id);
    println!("Namespace:   {}", model.namespace);
    println!("Protocol:    {}", model.protocol.as_str());
    if !model.additional_protocols.is_empty() {
        let names: Vec<&str> = model
            .additional_protocols
            .iter()
            .map(|p| p.as_str())
            .collect();
        println!("Additional:  {}", names.join(", "));
    }
    println!("Operations:  {}", model.operations.len());

    if let Some(ref ns) = model.xml_namespace {
        println!("XML NS:      {ns}");
    }

    // Count shapes by type
    let mut type_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    for shape in model.shapes.values() {
        if let Some(t) = shape.get("type").and_then(|v| v.as_str()) {
            *type_counts.entry(t.to_string()).or_default() += 1;
        }
    }
    println!("\nShape types:");
    let mut sorted: Vec<_> = type_counts.into_iter().collect();
    sorted.sort_by_key(|x| std::cmp::Reverse(x.1));
    for (t, count) in sorted {
        println!("  {t}: {count}");
    }

    Ok(())
}

fn cmd_ops(service: &str, models_dir: &Path, crates_dir: &Path) -> Result<()> {
    let model = load_model(service, models_dir)?;
    let crate_name = format!("winterbaume-{service}");
    let crate_dir = crates_dir.join(&crate_name);

    let existing_ops = if crate_dir.join("src").join("handlers.rs").exists() {
        let source = merge::read_handlers(&crate_dir)?;
        merge::extract_existing_operations(&source, model.protocol)
    } else {
        Vec::new()
    };

    let implemented = model
        .operations
        .iter()
        .filter(|op| existing_ops.contains(&op.name))
        .count();
    let total = model.operations.len();
    println!(
        "{crate_name}: {implemented}/{total} operations implemented ({} protocol)\n",
        model.protocol.as_str()
    );

    for op in &model.operations {
        let status = if existing_ops.contains(&op.name) {
            "  [x]"
        } else {
            "  [ ]"
        };
        let http_info = op
            .http
            .as_ref()
            .map(|h| format!(" ({} {})", h.method, h.uri))
            .unwrap_or_default();
        println!("{status} {}{http_info}", op.name);
    }

    Ok(())
}

fn cmd_inject(service: &str, models_dir: &Path, crates_dir: &Path, dry_run: bool) -> Result<()> {
    let model = load_model(service, models_dir)?;
    let crate_name = format!("winterbaume-{service}");
    let crate_dir = crates_dir.join(&crate_name);

    let source = merge::read_handlers(&crate_dir)
        .with_context(|| format!("Cannot read handlers for {crate_name}"))?;

    let existing_ops = merge::extract_existing_operations(&source, model.protocol);
    let all_arms = merge::extract_all_match_arms(&source, model.protocol);

    // Detect the error function used in the catch-all arm
    let default_error_fn = match model.protocol {
        Protocol::AwsJson1_0 | Protocol::AwsJson1_1 => "json_error_response",
        Protocol::RestJson1 => "rest_json_error",
        Protocol::AwsQuery | Protocol::Ec2Query | Protocol::RestXml => "MockResponse::error",
        Protocol::RpcV2Cbor => "cbor_error_response",
    };
    let error_fn =
        merge::detect_error_function(&source).unwrap_or_else(|| default_error_fn.to_string());

    let stub_arms = match model.protocol {
        Protocol::AwsJson1_0 | Protocol::AwsJson1_1 => {
            gen_aws_json::generate_stub_arms(&model, &all_arms, &crate_name, &error_fn)
        }
        Protocol::RestJson1 => {
            gen_rest_json::generate_stub_arms(&model, &all_arms, &crate_name, &error_fn)
        }
        Protocol::AwsQuery | Protocol::Ec2Query | Protocol::RestXml => {
            gen_aws_query::generate_stub_arms(&model, &all_arms, &crate_name, &error_fn)
        }
        Protocol::RpcV2Cbor => {
            gen_rpcv2_cbor::generate_stub_arms(&model, &all_arms, &crate_name, &error_fn)
        }
    };

    if stub_arms.trim().is_empty() {
        println!(
            "{crate_name}: all {} operations already have match arms",
            model.operations.len()
        );
        return Ok(());
    }

    let new_stub_count = model.operations.len().saturating_sub(all_arms.len());

    if dry_run {
        println!(
            "{crate_name}: would inject {new_stub_count} stub arms ({} existing, {} total)\n",
            existing_ops.len(),
            model.operations.len()
        );
        println!("{stub_arms}");
        return Ok(());
    }

    let new_source = merge::insert_stub_arms(&source, &stub_arms, model.protocol)?;
    merge::write_handlers(&crate_dir, &new_source)?;

    println!(
        "{crate_name}: injected {new_stub_count} stub arms ({} existing, {} total)",
        existing_ops.len(),
        model.operations.len()
    );

    Ok(())
}

fn cmd_scaffold(
    service: &str,
    models_dir: &Path,
    service_struct: Option<String>,
    state_type: Option<String>,
) -> Result<()> {
    let model = load_model(service, models_dir)?;
    let crate_name = format!("winterbaume-{service}");

    let service_struct =
        service_struct.unwrap_or_else(|| format!("{}Service", heck::AsUpperCamelCase(service)));
    let state_type =
        state_type.unwrap_or_else(|| format!("{}State", heck::AsUpperCamelCase(service)));

    let output = match model.protocol {
        Protocol::AwsJson1_0 | Protocol::AwsJson1_1 => {
            gen_aws_json::generate_handlers(&model, &crate_name, &service_struct, &state_type)
        }
        Protocol::RestJson1 => {
            gen_rest_json::generate_handlers(&model, &crate_name, &service_struct, &state_type)
        }
        Protocol::AwsQuery | Protocol::Ec2Query => {
            gen_aws_query::generate_handlers(&model, &crate_name, &service_struct, &state_type)
        }
        Protocol::RestXml => {
            gen_aws_query::generate_handlers(&model, &crate_name, &service_struct, &state_type)
        }
        Protocol::RpcV2Cbor => {
            gen_rpcv2_cbor::generate_handlers(&model, &crate_name, &service_struct, &state_type)
        }
    };

    println!("{output}");
    Ok(())
}

fn cmd_list_services() -> Result<()> {
    println!("{:<25} MODEL DIRECTORY", "CRATE SUFFIX");
    println!("{}", "-".repeat(50));
    for (crate_name, model_name) in discover::list_services() {
        println!("{crate_name:<25} {model_name}");
    }
    Ok(())
}

fn cmd_gen_serializers(
    service: &str,
    models_dir: &Path,
    output: Option<PathBuf>,
    split: bool,
    model_output: Option<PathBuf>,
    features_map: Option<PathBuf>,
) -> Result<()> {
    let model = load_model(service, models_dir)?;
    let crate_name = format!("winterbaume-{service}");

    let feature_info = match features_map {
        Some(path) => {
            let map = features::FeatureMap::load(&path, &model)?;
            let shape_features = features::compute_shape_features(&model, &map);
            Some((map, shape_features))
        }
        None => None,
    };

    if split {
        let wire_code = normalize_trailing_newline(gen_serializers::generate_wire_module_split(
            &model,
            &crate_name,
            feature_info.as_ref().map(|(m, _)| m),
        ));
        let model_code = normalize_trailing_newline(gen_serializers::generate_model_module(
            &model,
            &crate_name,
            feature_info.as_ref().map(|(_, s)| s),
        ));

        if let Some(wire_path) = output {
            let model_path =
                model_output.unwrap_or_else(|| wire_path.parent().unwrap().join("model.rs"));
            std::fs::write(&wire_path, &wire_code)
                .with_context(|| format!("Failed to write to {}", wire_path.display()))?;
            std::fs::write(&model_path, &model_code)
                .with_context(|| format!("Failed to write to {}", model_path.display()))?;
            rustfmt_file(&wire_path);
            rustfmt_file(&model_path);
            println!(
                "Generated {} serializers for {crate_name} -> {} + {}",
                model.operations.len(),
                wire_path.display(),
                model_path.display()
            );
            if let Some((map, _)) = &feature_info {
                eprintln!(
                    "Cargo features used (add to {crate_name}/Cargo.toml [features] section):"
                );
                let all = map.all_features();
                eprintln!("  full = {:?}", all);
                for f in all {
                    eprintln!("  {f} = []");
                }
            }
        } else {
            eprintln!("=== model.rs ===");
            print!("{model_code}");
            eprintln!("=== wire.rs ===");
            print!("{wire_code}");
        }
    } else {
        let code = normalize_trailing_newline(gen_serializers::generate_wire_module(
            &model,
            &crate_name,
            feature_info.as_ref().map(|(m, _)| m),
            feature_info.as_ref().map(|(_, s)| s),
        ));

        if let Some(path) = output {
            std::fs::write(&path, &code)
                .with_context(|| format!("Failed to write to {}", path.display()))?;
            rustfmt_file(&path);
            println!(
                "Generated {} serializers for {crate_name} -> {}",
                model.operations.len(),
                path.display()
            );
        } else {
            print!("{code}");
        }
    }

    Ok(())
}

fn cmd_inject_all(models_dir: &Path, crates_dir: &Path, dry_run: bool) -> Result<()> {
    let mut total_injected = 0;
    let mut total_existing = 0;
    let mut total_ops = 0;
    let mut errors = Vec::new();

    for (service, _) in discover::list_services() {
        let crate_name = format!("winterbaume-{service}");
        let crate_dir = crates_dir.join(&crate_name);

        // Skip services that don't have a handlers.rs yet
        if !crate_dir.join("src").join("handlers.rs").exists() {
            continue;
        }

        match cmd_inject_single(service, models_dir, crates_dir, dry_run) {
            Ok((existing, new, total)) => {
                total_existing += existing;
                total_injected += new;
                total_ops += total;
            }
            Err(e) => {
                errors.push(format!("{crate_name}: {e}"));
            }
        }
    }

    println!("\n--- Summary ---");
    println!("Total operations: {total_ops}");
    println!("Already implemented: {total_existing}");
    println!("Stubs injected: {total_injected}");
    if !errors.is_empty() {
        println!("\nErrors:");
        for e in &errors {
            println!("  {e}");
        }
    }

    Ok(())
}

fn cmd_inject_single(
    service: &str,
    models_dir: &Path,
    crates_dir: &Path,
    dry_run: bool,
) -> Result<(usize, usize, usize)> {
    let model = load_model(service, models_dir)?;
    let crate_name = format!("winterbaume-{service}");
    let crate_dir = crates_dir.join(&crate_name);

    let source = merge::read_handlers(&crate_dir)?;
    let existing_ops = merge::extract_existing_operations(&source, model.protocol);
    // Use all_arms (including stubs) to avoid double-injection
    let all_arms = merge::extract_all_match_arms(&source, model.protocol);

    // Detect the error function used in the catch-all arm
    let default_error_fn = match model.protocol {
        Protocol::AwsJson1_0 | Protocol::AwsJson1_1 => "json_error_response",
        Protocol::RestJson1 => "rest_json_error",
        Protocol::AwsQuery | Protocol::Ec2Query | Protocol::RestXml => "MockResponse::error",
        Protocol::RpcV2Cbor => "cbor_error_response",
    };
    let error_fn =
        merge::detect_error_function(&source).unwrap_or_else(|| default_error_fn.to_string());

    let stub_arms = match model.protocol {
        Protocol::AwsJson1_0 | Protocol::AwsJson1_1 => {
            gen_aws_json::generate_stub_arms(&model, &all_arms, &crate_name, &error_fn)
        }
        Protocol::RestJson1 => {
            gen_rest_json::generate_stub_arms(&model, &all_arms, &crate_name, &error_fn)
        }
        Protocol::AwsQuery | Protocol::Ec2Query | Protocol::RestXml => {
            gen_aws_query::generate_stub_arms(&model, &all_arms, &crate_name, &error_fn)
        }
        Protocol::RpcV2Cbor => {
            gen_rpcv2_cbor::generate_stub_arms(&model, &all_arms, &crate_name, &error_fn)
        }
    };

    let existing = existing_ops.len();
    let total = model.operations.len();
    let new_stubs = total.saturating_sub(all_arms.len());

    if stub_arms.trim().is_empty() {
        println!("{crate_name}: all {total} operations already have match arms");
        return Ok((existing, 0, total));
    }

    if !dry_run {
        let new_source = merge::insert_stub_arms(&source, &stub_arms, model.protocol)?;
        merge::write_handlers(&crate_dir, &new_source)?;
    }

    println!(
        "{crate_name}: {} stub{} {} ({existing} existing, {total} total)",
        new_stubs,
        if new_stubs == 1 { "" } else { "s" },
        if dry_run {
            "would be injected"
        } else {
            "injected"
        },
    );

    Ok((existing, new_stubs, total))
}
