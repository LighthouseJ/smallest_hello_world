use std::fs;
use std::process::Command;

fn main() {
    let target = std::env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let binary_path = format!("{}/release/hello", target);

    // Strip additional symbols
    Command::new("strip")
        .args(&["--strip-all", &binary_path])
        .status()
        .expect("Failed to strip binary");

    // Further reduce size using objcopy
    Command::new("objcopy")
        .args(&["--strip-unneeded", &binary_path])
        .status()
        .expect("Failed to optimize binary with objcopy");

    
    // Get and print the file size
    if let Ok(metadata) = fs::metadata(&binary_path) {
        println!("cargo:warning=Final executable size: {} bytes", metadata.len());
    } else {
        println!("cargo:warning=Could not determine binary size.");
    }

    println!("cargo:rerun-if-changed=build.rs");

}