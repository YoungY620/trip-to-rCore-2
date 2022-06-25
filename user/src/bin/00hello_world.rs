#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::get_task_info;

#[no_mangle]
fn main() -> i32 {
    println!("Hello, world!");
    let a = get_task_info();
    println!("task_info return: {}", a);
    0
}
