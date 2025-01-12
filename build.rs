use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = Path::new(&out_dir).parent().unwrap().parent().unwrap().join("deps");

    fs::create_dir_all(&target_dir).unwrap();

    let dlls = ["libvosk.dll", "libstdc++-6.dll", "libgcc_s_seh-1.dll", "libwinpthread-1.dll"];
    for dll in dlls.iter() {
        fs::copy(format!("./libs/{}", dll), target_dir.join(dll)).unwrap();
    }


    println!("cargo:rustc-link-lib=dylib=vosk");
    println!("cargo:rustc-link-search=./libs");
}