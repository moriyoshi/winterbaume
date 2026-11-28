//! `cfg(lambda_vm)`: the `lambda-vm` feature on a target that has the Lambda
//! microVM executor ( Linux, or macOS on arm64 ).  The dependency is
//! target-specific, so the feature alone does not say whether it is there.

fn main() {
    println!("cargo::rustc-check-cfg=cfg(lambda_vm)");
    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let supported = os == "linux" || (os == "macos" && arch == "aarch64");
    if supported && std::env::var_os("CARGO_FEATURE_LAMBDA_VM").is_some() {
        println!("cargo::rustc-cfg=lambda_vm");
    }
}
