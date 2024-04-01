#![no_std]
#![no_main]
#![windows_subsystem = "windows"] // necessary to remove the weird console window that appears alongside the real GUI on Windows

use core::panic::PanicInfo;

use windows_sys::{
    w,
    Win32::{
        System::Threading::ExitProcess,
        UI::WindowsAndMessaging::{
            MB_ICONERROR,
            MB_YESNOCANCEL,
            MessageBoxW,
        },
    },
};

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    unsafe {
        MessageBoxW(
            0,
            w!("Hi, I am an Albanian virus but because of poor technology in my country unfortunately I am not able to harm your computer. Please be so kind to delete one of your important files yourself and then forward me to other users. Many thanks for your cooperation! Best regards,Albanian virus"),
            w!("Virus Alert !"),
            MB_YESNOCANCEL | MB_ICONERROR,
        );
        ExitProcess(0);
    }
}

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    unsafe {
        ExitProcess(1);
    }
}
