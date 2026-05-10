//! End-to-end smoke test: every spec under `crates/winterbaume-terraform/specs`
//! re-emits to byte-equal output as the committed `generated/<service>.rs`.
//!
//! Run with `cargo test -p tf-converter-codegen --test check`. Failures point
//! reviewers at the stale generated files and tell them which `gen` command
//! to rerun.

use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .and_then(|p| p.parent())
        .expect("tools/tf-converter-codegen has a workspace root")
        .to_path_buf()
}

#[test]
fn check_passes_for_all_committed_specs() {
    let root = workspace_root();
    let bin = env!("CARGO_BIN_EXE_tf-converter-codegen");
    let output = Command::new(bin)
        .current_dir(&root)
        .arg("check")
        .output()
        .expect("invoke tf-converter-codegen check");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "tf-converter-codegen check failed.\nstdout: {stdout}\nstderr: {stderr}"
    );
}
