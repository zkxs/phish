#![no_std] // use `core` instead of `std`: https://doc.rust-lang.org/reference/names/preludes.html#the-no_std-attribute
#![no_main] // Don't emit the `main` symbol: https://doc.rust-lang.org/reference/crates-and-source-files.html#the-no_main-attribute
#![windows_subsystem = "windows"] // necessary to remove the weird console window that appears alongside the real GUI on Windows

use core::hint::unreachable_unchecked;
use core::panic::PanicInfo;

use windows_sys::{
    s,
    Win32::UI::WindowsAndMessaging::{
        MB_ICONERROR,
        MB_YESNOCANCEL,
        MessageBoxA,
    },
};

/// mainCRTStartup is the default entry point
/// [expected by the linker](https://learn.microsoft.com/en-us/cpp/build/reference/entry-entry-point-symbol?view=msvc-170).
/// Because we're using no_main we must create this ourselves. Note that
/// [extern "system"](https://doc.rust-lang.org/reference/items/external-blocks.html) uses the
/// stdcall calling convention as required on Win32
#[no_mangle] // Don't fuck up the symbol name: https://doc.rust-lang.org/reference/abi.html#the-no_mangle-attribute
pub extern "system" fn mainCRTStartup() -> u32 {
    unsafe {
        // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-messageboxa
        MessageBoxA(
            0, // intentional null pointer
            s!("Hi, I am an Albanian virus but because of poor technology in my country unfortunately I am not able to harm your computer. Please be so kind to delete one of your important files yourself and then forward me to other users. Many thanks for your cooperation! Best regards,Albanian virus"),
            s!("Virus Alert !"),
            MB_YESNOCANCEL | MB_ICONERROR,
        );
    }
    1
}

/// We must define our own panic handler under no_std, but we never panic so just declare it to be unreachable
#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    unsafe {
        unreachable_unchecked();
    }
}
