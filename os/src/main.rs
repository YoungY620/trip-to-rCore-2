#![no_std]
#![no_main]
#![feature(global_asm)]  // 对于不稳定的 feature 需要
#![feature(asm)]
#![feature(panic_info_message)] // for `PanicInfo::message`

mod sbi;
#[macro_use]
mod console;    // 顺序, `lang_item` 中要使用 `console` 则需要先声明
mod lang_item;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    println!("Hello, world!");
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}
