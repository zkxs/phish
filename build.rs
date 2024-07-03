use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS");
    let target_env = env::var("CARGO_CFG_TARGET_ENV");
    let out_dir =  env::var("OUT_DIR").unwrap();
    if Ok("windows") == target_os.as_deref() && Ok("msvc") == target_env.as_deref() {
        // we should be using link.exe for this target, but we've done .cargo/config.toml hijinks to change it to crinkler
        // default flags cargo adds: "/NOLOGO" "/LARGEADDRESSAWARE" "/SAFESEH" "/NXCOMPAT" "/SUBSYSTEM:windows" "/ENTRY:mainCRTStartup" "/OPT:REF,ICF" "/DEBUG:NONE"
        // linker flag documentation: https://github.com/runestubbe/Crinkler/blob/master/doc/manual.txt
        println!("cargo:rustc-link-arg-bins=/NODEFAULTLIB");
        println!("cargo:rustc-link-arg-bins=/COMPMODE:VERYSLOW");
        println!("cargo:rustc-link-arg-bins=/HASHSIZE:100");
        println!("cargo:rustc-link-arg-bins=/HASHTRIES:1024");
        println!("cargo:rustc-link-arg-bins=/TINYHEADER");
        println!("cargo:rustc-link-arg-bins=/TINYIMPORT");
        println!("cargo:rustc-link-arg-bins=/ORDERTRIES:128");
        println!("cargo:rustc-link-arg-bins=/UNSAFEIMPORT");
        println!("cargo:rustc-link-arg-bins=/NOINITIALIZERS");
        println!("cargo:rustc-link-arg-bins=/UNALIGNCODE");
        println!("cargo:rustc-link-arg-bins=/REPORT:{}/crinkler-report.html", out_dir);
    }

    // Avoid rerunning the build script every time. Source: https://github.com/ChrisDenton/rust/blob/1.65.0/compiler/rustc/build.rs
    // Docs: https://doc.rust-lang.org/cargo/reference/build-scripts.html
    println!("cargo:rerun-if-changed=build.rs");
}
