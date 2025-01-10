use std::{env, path::PathBuf, process::Command};

fn main() {
    let vosk_lib_path = find_vosk_lib().expect("Vosk library not found");

    println!("cargo:rustc-link-search=native={}", vosk_lib_path.display());
    println!("cargo:rustc-link-lib=dylib=vosk");

    println!("cargo:rerun-if-env-changed=VOSK_LIB_DIR");
    println!("cargo:rerun-if-env-changed=VOSK_INCLUDE_DIR")
}

fn find_vosk_lib() -> Option<PathBuf> {
    if let Ok(lib_dir) = env::var("VOSK_LIB_DIR") {
        return Some(PathBuf::from(lib_dir));
    }

    if cfg!(target_os = "linux") {
        if let Ok(output) = Command::new("pkg-config").args(&["--variable=libdir", "vosk"]).output() {
            if output.status.success() {
                let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                return Some(PathBuf::from(path));
            }
        }
    }

    None
}