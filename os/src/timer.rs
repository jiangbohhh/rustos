use crate::sbi::set_timer;
use riscv::register::{sie, time};

// 当前已触发时钟中断次数
pub static mut TICKS: usize = 0;
// 触发时钟中断间隔，数值一般为CPU 频率的 1%，防止过多占用 CPU 资源
static TIMEBASE: u64 = 100000;

pub fn init() {
    unsafe {
        // 初始化时钟中断触发次数
        TICKS = 0;
        // 设置 sie 的 TI 使能 STIE 位
        sie::set_stimer();
    }

    clock_set_next_event();
    println!("++++ setup timer!     ++++");
}

pub fn clock_set_next_event() {
    set_timer(get_cycle() + TIMEBASE);
}

fn get_cycle() -> u64 {
    time::read() as u64
}
