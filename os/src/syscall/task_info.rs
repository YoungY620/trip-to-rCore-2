//!task info
use crate::batch::get_app_info;


pub fn get_task_info() -> isize{
    let mgr = get_app_info();
    mgr as isize
}