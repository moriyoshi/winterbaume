mod codegen;
mod ingest;
mod spec;

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "tf-converter-codegen",
    about = "Generate serde-driven Terraform converter projection helpers from per-service TOML specs"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Path to the directory holding per-service TOML specs.
    #[arg(
        long,
        default_value = "crates/winterbaume-terraform/specs",
        global = true
    )]
    specs_dir: PathBuf,

    /// Path to the directory where generated projection modules are written.
    /// Files named `<service>.rs` go directly into this directory; the parent
    /// `winterbaume-tfstate-resource-models` crate's `lib.rs` declares the
    /// modules.
    #[arg(
        long,
        default_value = "crates/winterbaume-tfstate-resource-models/src",
        global = true
    )]
    output_dir: PathBuf,
}

#[derive(Subcommand)]
enum Command {
    /// Generate `generated/<service>.rs` from `specs/<service>.toml`.
    Gen {
        /// Service name (e.g., "sqs"). Matches the spec file stem.
        service: String,
    },
    /// Run `gen` for every spec under `--specs-dir`.
    GenAll,
    /// Re-emit every spec to a temp tree and diff against committed files.
    /// Exits non-zero if any committed file is stale.
    Check,
    /// List the specs discovered under `--specs-dir`.
    List,
    /// Bootstrap a TOML spec from an existing converter source by parsing
    /// the `optional_str` / `require_str` / `extract_tags` patterns. The
    /// result is written to `--specs-dir/<service>.toml` and is intended
    /// as a starting point for a human reviewer.
    Ingest {
        /// Service name (must match `<service>.rs` under `converters/`).
        service: String,
        /// Override the converter directory.
        #[arg(long, default_value = "crates/winterbaume-terraform/src/converters")]
        converters_dir: PathBuf,
        /// Print the generated spec to stdout instead of writing it.
        #[arg(long)]
        dry_run: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Gen { service } => run_gen(&cli.specs_dir, &cli.output_dir, &service),
        Command::GenAll => run_gen_all(&cli.specs_dir, &cli.output_dir),
        Command::Check => run_check(&cli.specs_dir, &cli.output_dir),
        Command::List => run_list(&cli.specs_dir),
        Command::Ingest {
            service,
            converters_dir,
            dry_run,
        } => run_ingest(&cli.specs_dir, &converters_dir, &service, dry_run),
    }
}

fn run_gen(specs_dir: &Path, output_dir: &Path, service: &str) -> Result<()> {
    let spec_path = specs_dir.join(format!("{service}.toml"));
    let spec = spec::load(&spec_path)
        .with_context(|| format!("loading spec from {}", spec_path.display()))?;
    let rendered = codegen::render(&spec)?;
    let out_path = output_dir.join(format!("{service}.rs"));
    std::fs::create_dir_all(output_dir)
        .with_context(|| format!("creating {}", output_dir.display()))?;
    std::fs::write(&out_path, rendered)
        .with_context(|| format!("writing {}", out_path.display()))?;
    rustfmt_file(&out_path);
    println!("wrote {}", out_path.display());
    Ok(())
}

fn run_gen_all(specs_dir: &Path, output_dir: &Path) -> Result<()> {
    for service in discover_services(specs_dir)? {
        run_gen(specs_dir, output_dir, &service)?;
    }
    Ok(())
}

fn run_check(specs_dir: &Path, output_dir: &Path) -> Result<()> {
    let mut stale: Vec<PathBuf> = Vec::new();
    for service in discover_services(specs_dir)? {
        let spec_path = specs_dir.join(format!("{service}.toml"));
        let spec = spec::load(&spec_path)?;
        let mut rendered = codegen::render(&spec)?;
        rendered = rustfmt_string(&rendered);
        let committed_path = output_dir.join(format!("{service}.rs"));
        let committed = std::fs::read_to_string(&committed_path).unwrap_or_default();
        if committed != rendered {
            stale.push(committed_path);
        }
    }
    if stale.is_empty() {
        println!("all generated files are fresh");
        Ok(())
    } else {
        for path in &stale {
            eprintln!("stale: {}", path.display());
        }
        bail!(
            "{} generated file(s) are stale; rerun `tf-converter-codegen gen-all`",
            stale.len()
        );
    }
}

fn run_ingest(specs_dir: &Path, converters_dir: &Path, service: &str, dry_run: bool) -> Result<()> {
    let toml = ingest::ingest(service, converters_dir)?;
    if dry_run {
        print!("{toml}");
        return Ok(());
    }
    std::fs::create_dir_all(specs_dir)
        .with_context(|| format!("creating {}", specs_dir.display()))?;
    let out_path = specs_dir.join(format!("{service}.toml"));
    std::fs::write(&out_path, toml).with_context(|| format!("writing {}", out_path.display()))?;
    println!("wrote {}", out_path.display());
    Ok(())
}

fn run_list(specs_dir: &Path) -> Result<()> {
    for service in discover_services(specs_dir)? {
        println!("{service}");
    }
    Ok(())
}

fn discover_services(specs_dir: &Path) -> Result<Vec<String>> {
    let mut out = Vec::new();
    if !specs_dir.exists() {
        return Ok(out);
    }
    for entry in
        std::fs::read_dir(specs_dir).with_context(|| format!("reading {}", specs_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("toml")
            && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
        {
            out.push(stem.to_string());
        }
    }
    out.sort();
    Ok(out)
}

fn rustfmt_file(path: &Path) {
    let _ = std::process::Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .arg(path)
        .status();
}

fn rustfmt_string(s: &str) -> String {
    use std::io::Write;
    let mut child = match std::process::Command::new("rustfmt")
        .arg("--edition")
        .arg("2021")
        .arg("--emit")
        .arg("stdout")
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(_) => return s.to_string(),
    };
    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(s.as_bytes());
    }
    let output = match child.wait_with_output() {
        Ok(o) => o,
        Err(_) => return s.to_string(),
    };
    if output.status.success() {
        String::from_utf8(output.stdout).unwrap_or_else(|_| s.to_string())
    } else {
        s.to_string()
    }
}
