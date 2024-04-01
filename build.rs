use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS");
    let target_env = env::var("CARGO_CFG_TARGET_ENV");
    let profile = env::var("PROFILE").unwrap();
    if Ok("windows") == target_os.as_deref() && Ok("msvc") == target_env.as_deref() && profile != "debug" {
        // we should be using link.exe for this target
        // default flags: "/NOLOGO" "/LARGEADDRESSAWARE" "/SAFESEH" "/NXCOMPAT" "/SUBSYSTEM:windows" "/ENTRY:mainCRTStartup" "/OPT:REF,ICF" "/DEBUG:NONE"
        // documentation: https://learn.microsoft.com/en-us/cpp/build/reference/linker-options?view=msvc-170

        // flags that actually reduce binary size:
        println!("cargo:rustc-link-arg-bins=/ALIGN:4"); // decrease PE alignment http://www.phreedom.org/research/tinype/
        println!("cargo:rustc-link-arg-bins=/IGNORE:4108"); // ignore warning from changing alignment

        // flags that looked like they might help, but actually don't:
        //println!("cargo:rustc-link-arg-bins=/merge:.rdata=.text"); // has no impact on binary size... appears to create an .idata section
        //println!("cargo:rustc-link-arg-bins=/FUNCTIONPADMIN:0"); // makes the output LARGER for some reason
        //println!("cargo:rustc-link-arg-bins=/NOFUNCTIONPADSECTION:.text"); // allegedly is only for x64, has no impact on binary size
        //println!("cargo:rustc-link-arg-bins=/NOFUNCTIONPADSECTION:.rdata"); // allegedly is only for x64, has no impact on binary size
        //println!("cargo:rustc-link-arg-bins=/NOFUNCTIONPADSECTION:.reloc"); // allegedly is only for x64, has no impact on binary size
    }

    // Avoid rerunning the build script every time. Source: https://github.com/ChrisDenton/rust/blob/1.65.0/compiler/rustc/build.rs
    // Docs: https://doc.rust-lang.org/cargo/reference/build-scripts.html
    println!("cargo:rerun-if-changed=build.rs");
}
