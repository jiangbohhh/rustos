use riscv::register:: {
    scause,
    sepc,
    stvec,
    sscratch
};

use crate::context::TrapFrame;

global_asm!(include_str!("trap/trap.asm"));


pub fn init() {
    unsafe {
        extern "C" {
            // 中断处理总入口
            fn __alltraps();
        }
        // 由于是内核态 需要把 sscratch
        sscratch::write(0);
        stvec::write(__alltraps as usize, stvec::TrapMode::Direct);
    }
    println!("++++ interrupt setup ++++");
}

#[no_mangle]
pub fn rust_trap(tf: &mut TrapFrame) {
    println!("rust_trap!");
    tf.sepc += 2;
}

