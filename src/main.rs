#![no_std]
#![no_main]
#![windows_subsystem = "windows"] // necessary to remove the weird console window that appears alongside the real GUI on Windows

use core::panic::PanicInfo;

use windows_sys::{
    w,
    Win32::{
        Foundation::GetLastError,
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
    let result = unsafe {
        MessageBoxW(
            0,
            w!("Hi, I am an Albanian virus but because of poor technology in my country unfortunately I am not able to harm your computer. Please be so kind to delete one of your important files yourself and then forward me to other users. Many thanks for your cooperation! Best regards,Albanian virus"),
            w!("Virus Alert !"),
            MB_YESNOCANCEL | MB_ICONERROR,
        )
    };
    let return_code = match result {
        0 => unsafe { GetLastError() }, // look up the Windows error code as per https://learn.microsoft.com/en-us/windows/win32/debug/system-error-codes--0-499-
        _ => 0, // I could get the pressed button as per https://github.com/Nerixyz/win-msgbox/blob/master/src/yes_no_cancel.rs, but we really don't care in this scenario
    };
    unsafe {
        ExitProcess(return_code)
    }
}

#[panic_handler]
fn panic(_: &PanicInfo<'_>) -> ! {
    unsafe {
        ExitProcess(1);
    }
}
