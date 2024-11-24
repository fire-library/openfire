// use std::env;
// use std::path::Path;

// const DEFAULT_CLANG_VERSION: &str = "19";

fn main() {
    // println!("cargo:rustc-link-search=native=/Users/jamiemaclean/workspace/rust/fire_lib/desktop/src-python");
    // println!("cargo:rustc-link-lib=hello"); // The name of your library, without the `lib` prefix
    tauri_build::build()
}
