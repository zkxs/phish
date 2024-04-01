use std::{env, path::Path};

fn main() {
    // add a Windows manifest as per https://dev.to/carey/embed-a-windows-manifest-in-your-rust-program-26j2
    // this prevents Windows from thinking that "this program might not have run correctly" and that it might need compatibility mode
    if env::var("TARGET").expect("target").ends_with("windows-msvc") {
        let manifest = Path::new("app.manifest").canonicalize().unwrap();
        println!("cargo:rustc-link-arg-bins=/MANIFEST:EMBED");
        println!("cargo:rustc-link-arg-bins=/MANIFESTINPUT:{}", manifest.display());
        println!("cargo:rerun-if-changed=hello.exe.manifest");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
