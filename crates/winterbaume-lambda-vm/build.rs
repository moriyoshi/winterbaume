//! Embeds the guest agent, so that a binary using this crate ( the server,
//! which is its own VM helper ) needs no separate agent file.
//!
//! In order:
//! 1. `WINTERBAUME_LAMBDA_VM_AGENT` names a prebuilt agent: that one.
//! 2. The agent's source is beside this crate ( a repository checkout ) and
//!    the Rust target for the guest is installed: it is built here, for the
//!    target's architecture ( guests run on the host's ).
//! 3. Otherwise nothing is embedded, with a build warning; the agent is then
//!    looked up on disk at run time.  With `WINTERBAUME_LAMBDA_VM_REQUIRE_AGENT=1`
//!    ( release builds ) that is an error instead.

use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(embedded_agent)");
    println!("cargo::rerun-if-env-changed=WINTERBAUME_LAMBDA_VM_AGENT");
    println!("cargo::rerun-if-env-changed=WINTERBAUME_LAMBDA_VM_REQUIRE_AGENT");

    if let Some(path) = std::env::var_os("WINTERBAUME_LAMBDA_VM_AGENT").filter(|p| !p.is_empty()) {
        let path = PathBuf::from(path);
        let path = std::fs::canonicalize(&path).unwrap_or_else(|e| {
            panic!(
                "WINTERBAUME_LAMBDA_VM_AGENT names {}, which cannot be read: {e}",
                path.display()
            )
        });
        return embed(&path);
    }

    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let guest = match (os.as_str(), arch.as_str()) {
        ("linux", "x86_64") => "x86_64-unknown-linux-gnu",
        ("linux", "aarch64") | ("macos", "aarch64") => "aarch64-unknown-linux-gnu",
        // No libkrun here: nothing to embed.
        _ => return,
    };
    let manifest_dir = PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let source = manifest_dir.join("../winterbaume-lambda-vm-agent");
    if !source.join("Cargo.toml").is_file() {
        // A published copy of this crate: the agent's source is not there.
        return;
    }
    for input in ["src", "Cargo.toml", "Cargo.lock", ".cargo/config.toml"] {
        println!("cargo::rerun-if-changed={}", source.join(input).display());
    }
    if !target_installed(guest) {
        return not_embedded(&format!(
            "install the {guest} Rust target ( rustup target add {guest} ) or set \
             WINTERBAUME_LAMBDA_VM_AGENT"
        ));
    }
    match build_agent(&source, guest) {
        Ok(binary) => embed(&binary),
        Err(message) => not_embedded(&message),
    }
}

fn not_embedded(why: &str) {
    if std::env::var("WINTERBAUME_LAMBDA_VM_REQUIRE_AGENT").as_deref() == Ok("1") {
        panic!("the guest agent cannot be embedded: {why}");
    }
    println!("cargo::warning=the guest agent is not embedded: {why}");
}

fn embed(path: &Path) {
    println!("cargo::rerun-if-changed={}", path.display());
    println!(
        "cargo::rustc-env=WINTERBAUME_EMBEDDED_AGENT={}",
        path.display()
    );
    println!("cargo::rustc-cfg=embedded_agent");
}

fn target_installed(target: &str) -> bool {
    let rustc = std::env::var_os("RUSTC").unwrap_or_else(|| "rustc".into());
    Command::new(rustc)
        .args(["--print", "sysroot"])
        .output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| PathBuf::from(String::from_utf8_lossy(&out.stdout).trim()))
        .is_some_and(|sysroot| {
            sysroot
                .join("lib/rustlib")
                .join(target)
                .join("lib")
                .is_dir()
        })
}

/// Build the agent with its own configuration ( `.cargo/config.toml` links it
/// statically with rust-lld ) into this build's output directory.
fn build_agent(source: &Path, guest: &str) -> Result<PathBuf, String> {
    let cargo = std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into());
    let target_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("agent");
    let mut command = Command::new(cargo);
    command
        .current_dir(source)
        .args(["build", "--release", "--target", guest, "--target-dir"])
        .arg(&target_dir);
    // The outer build's flags and target settings are not the agent's: they
    // would replace the rustflags of its configuration.
    for variable in [
        "RUSTFLAGS",
        "CARGO_ENCODED_RUSTFLAGS",
        "CARGO_BUILD_RUSTFLAGS",
        "CARGO_TARGET_DIR",
        "CARGO_BUILD_TARGET",
        "CARGO_BUILD_TARGET_DIR",
        "CARGO_MAKEFLAGS",
        "MAKEFLAGS",
        "MFLAGS",
    ] {
        command.env_remove(variable);
    }
    for (key, _) in std::env::vars_os() {
        let key = key.to_string_lossy().to_string();
        if (key.starts_with("CARGO_TARGET_") && key.ends_with("_RUSTFLAGS"))
            || key.starts_with("CARGO_PROFILE_")
            || key.starts_with("CARGO_FEATURE_")
            || key.starts_with("CARGO_CFG_")
        {
            command.env_remove(key);
        }
    }
    let output = command
        .output()
        .map_err(|e| format!("cannot run cargo: {e}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let tail: Vec<&str> = stderr.lines().rev().take(5).collect();
        let tail: Vec<&str> = tail.into_iter().rev().collect();
        return Err(format!("building it failed: {}", tail.join(" | ")));
    }
    let binary = target_dir
        .join(guest)
        .join("release")
        .join("winterbaume-lambda-vm-agent");
    if binary.is_file() {
        Ok(binary)
    } else {
        Err(format!("{} was not produced", binary.display()))
    }
}
