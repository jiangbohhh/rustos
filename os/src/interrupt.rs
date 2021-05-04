use riscv::register::{
    scause::{self, Exception, Interrupt, Trap},
    sepc, sscratch, sstatus, stvec,
};

use crate::context::TrapFrame;
use crate::timer::{clock_set_next_event, TICKS};

global_asm!(include_str!("trap/trap.asm"));

pub fn init() {
    unsafe {
        extern "C" {
            // 中断处理总入口
            fn __alltraps();
        }
        // 由于是内核态 需要把 sscratch置为 0，来表示在 S 态产生的中断
        sscratch::write(0);
        // 使用 Direct 模式，将中断处理入口统一设置成 __alltraps
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
        // 设置 sstatus 的 sie 位
        sstatus::set_sie();
    }
    println!("++++ interrupt setup ++++");
}

#[no_mangle]
pub fn rust_trap(tf: &mut TrapFrame) {
    match tf.scause.cause() {
        // 断点中断
        Trap::Exception(Exception::Breakpoint) => breakpoint(&mut tf.sepc),
        // S 态时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => super_timer(),
        _ => panic!("undefined trap!"),
    }
}

fn breakpoint(sepc: &mut usize) {
    println!("a breakpoint set @0x{:x}", sepc);
    *sepc += 2;
}

fn super_timer() {
    clock_set_next_event();
    unsafe {
        TICKS += 1;
        if TICKS == 100 {
            TICKS = 0;
            println!("* 100 ticks *")
        }
    }
}
