//! # 全局属性
//! - `#![no_std]`
//! 禁用标准库
#![no_std]

//!
//! - `#![no_main]`
//! 不使用`main`函数等全部 Rust-level 入口点作为程序入口
#![no_main]

use core::panic::PanicInfo;

// 这个函数在panic时被调用
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// 覆盖 crt_0 中的 _start 函数
#[no_mangle] // 不重整函数名
pub extern "C" fn _start() -> ! {
    // 编译器会寻找一个名为`_start`的函数， 所以这个函数就是入口点
    // 默认命名为`_start`
    loop {}
}


