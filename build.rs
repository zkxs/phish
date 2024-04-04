use std::env;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS");
    let target_env = env::var("CARGO_CFG_TARGET_ENV");
    let profile = env::var("PROFILE").unwrap();
    if Ok("windows") == target_os.as_deref() && Ok("msvc") == target_env.as_deref() {
        // we should be using link.exe for this target
        // default flags cargo adds: "/NOLOGO" "/LARGEADDRESSAWARE" "/SAFESEH" "/NXCOMPAT" "/SUBSYSTEM:windows" "/ENTRY:mainCRTStartup" "/OPT:REF,ICF" "/DEBUG:NONE"
        // linker flag documentation: https://learn.microsoft.com/en-us/cpp/build/reference/linker-options?view=msvc-170

        // decrease PE alignment http://www.phreedom.org/research/tinype/
        if profile == "debug" {
            println!("cargo:rustc-link-arg-bins=/ALIGN:16"); // no idea why this is 16 minimum for debug builds
        } else {
            println!("cargo:rustc-link-arg-bins=/ALIGN:4"); // no idea why this is 4 minimum for release builds
        }
        println!("cargo:rustc-link-arg-bins=/IGNORE:4108"); // ignore warning from changing alignment

        // undocumented flag that removes some debug info from the .rdata section: https://stackoverflow.com/questions/45538668/remove-image-debug-directory-from-rdata-section/45546715#45546715
        println!("cargo:rustc-link-arg-bins=/EMITPOGOPHASEINFO");

        // disable ASLR to remove the relocation table
        println!("cargo:rustc-link-arg-bins=/DYNAMICBASE:NO");

        // use minimal MS-DOS stub, which as per https://github.com/mcountryman/min-sized-rust-windows/pull/7 is just the first 64 bytes of the DOS header
        println!("cargo:rustc-link-arg-bins=/STUB:stub.exe");

        // linker flags that looked like they might help, but actually don't:
        //println!("cargo:rustc-link-arg-bins=/WINMD:NO");
        //println!("cargo:rustc-link-arg-bins=/SAFESEH:NO");
        //println!("cargo:rustc-link-arg-bins=/INCREMENTAL:NO");
        //println!("cargo:rustc-link-arg-bins=/MANIFESTUAC:NO");
        //println!("cargo:rustc-link-arg-bins=/TLBID:1");
        //println!("cargo:rustc-link-arg-bins=/MANIFEST:NO");
        //println!("cargo:rustc-link-arg-bins=/RELEASE"); // something about a device driver checksum, appears to do nothing
        //println!("cargo:rustc-link-arg-bins=/FILEALIGN:1"); // has something to do with section alignment, but doesn't have any effect on binary size
        //println!("cargo:rustc-link-arg-bins=/NODEFAULTLIB"); //removes all default libraries from the list of libraries link.exe searches when resolving external references. However, it appears to do nothing in our case
        //println!("cargo:rustc-link-arg-bins=/merge:.rdata=.text"); // has no impact on binary size... appears to create an .idata section
        //println!("cargo:rustc-link-arg-bins=/merge:.reloc=.text"); // cannot be merged
        //println!("cargo:rustc-link-arg-bins=/merge:.idata=.text"); // cannot be merged
        //println!("cargo:rustc-link-arg-bins=/FUNCTIONPADMIN:0"); // makes the output LARGER for some reason
        //println!("cargo:rustc-link-arg-bins=/NOFUNCTIONPADSECTION:.text"); // allegedly is only for x64, has no impact on binary size
        //println!("cargo:rustc-link-arg-bins=/NOFUNCTIONPADSECTION:.rdata"); // allegedly is only for x64, has no impact on binary size
        //println!("cargo:rustc-link-arg-bins=/NOFUNCTIONPADSECTION:.reloc"); // allegedly is only for x64, has no impact on binary size
    }

    // Avoid rerunning the build script every time. Source: https://github.com/ChrisDenton/rust/blob/1.65.0/compiler/rustc/build.rs
    // Docs: https://doc.rust-lang.org/cargo/reference/build-scripts.html
    println!("cargo:rerun-if-changed=build.rs");
}
