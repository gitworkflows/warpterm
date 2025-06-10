use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    // Set build-time environment variables
    println!(
        "cargo:rustc-env=BUILD_TIME={}",
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    );
    println!("cargo:rustc-env=GIT_HASH={}", get_git_hash());

    // Platform-specific configurations
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    match target_os.as_str() {
        "macos" => {
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
            println!("cargo:rustc-link-lib=framework=Security");
        }
        "linux" => {
            println!("cargo:rustc-link-lib=X11");
        }
        "windows" => {
            println!("cargo:rustc-link-lib=user32");
            println!("cargo:rustc-link-lib=kernel32");
        }
        _ => {}
    }

    // Generate version info
    generate_version_info();
}

fn get_git_hash() -> String {
    std::process::Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .ok()
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string())
}

fn generate_version_info() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("version.rs");

    let version_info = format!(
        r#"
pub const VERSION: &str = "{}";
pub const BUILD_TIME: &str = "{}";
pub const GIT_HASH: &str = "{}";
pub const TARGET: &str = "{}";
"#,
        env::var("CARGO_PKG_VERSION").unwrap(),
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
        get_git_hash(),
        env::var("TARGET").unwrap_or_else(|_| "unknown".to_string())
    );

    fs::write(&dest_path, version_info).unwrap();
}
