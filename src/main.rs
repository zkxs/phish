#![no_main]
#![windows_subsystem = "windows"] // necessary to remove the weird console window that appears alongside the real GUI on Windows

use win_msgbox::{Modal, Okay};
use windows_sys::w;

#[no_mangle]
pub fn main(_argc: i32, _argv: *const *const u8) {
    win_msgbox::MessageBox::<Okay>::new(w!("hacked"))
        .icon(win_msgbox::Icon::Error)
        .modal(Modal::System)
        .title(w!("Uh Oh"))
        .show()
        .unwrap();
}
