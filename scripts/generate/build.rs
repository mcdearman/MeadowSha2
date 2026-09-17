//! Finds the `sha2` source that Cargo fetched: its constants, which `main.rs`
//! writes out, and the portable compression functions and the padding, which
//! `main.rs` fingerprints since `src/` ports them by hand.

use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let mut sources = String::new();
    let dir = upstream_dir("sha2");
    println!("cargo:rustc-env=UPSTREAM_DIR={}", dir.display());
    for file in [
        "src/core_api.rs",
        "src/sha256/soft_compact.rs",
        "src/sha512/soft_compact.rs",
    ] {
        let path = dir.join(file);
        sources.push_str(
            &std::fs::read_to_string(&path)
                .unwrap_or_else(|e| panic!("could not read {}: {e}", path.display())),
        );
        println!("cargo:rerun-if-changed={}", path.display());
    }
    std::fs::write(out.join("sources.rs.txt"), sources).unwrap();
    // `include!` takes no inner attributes, so they are dropped.
    let consts = std::fs::read_to_string(dir.join("src/consts.rs")).unwrap();
    let consts: String = consts
        .lines()
        .filter(|l| !l.starts_with("#!["))
        .map(|l| format!("{l}\n"))
        .collect();
    std::fs::write(out.join("consts.rs"), consts).unwrap();
    println!(
        "cargo:rerun-if-changed={}",
        dir.join("src/consts.rs").display()
    );
    println!("cargo:rerun-if-changed=Cargo.toml");
}

/// Where Cargo put the package `name` this build depends on.
fn upstream_dir(name: &str) -> PathBuf {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let manifest = Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).join("Cargo.toml");
    let out = Command::new(cargo)
        .args(["metadata", "--format-version", "1", "--manifest-path"])
        .arg(&manifest)
        .output()
        .expect("could not run `cargo metadata`");
    assert!(out.status.success(), "`cargo metadata` failed");
    let meta: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    let pkg = meta["packages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["name"] == name)
        .unwrap_or_else(|| panic!("{name} is not among the dependencies"));
    Path::new(pkg["manifest_path"].as_str().unwrap())
        .parent()
        .unwrap()
        .to_path_buf()
}
