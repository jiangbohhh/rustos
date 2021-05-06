use crate::consts::*;
global_asm!(include_str!("boot/entry64.asm"));

use crate::memory::{alloc_frame, dealloc_frame};

#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    extern "C" {
        fn end();
    }
    println!("kernel end vaddr = {:#x}", end as usize);
    // 物理页内存为 [PPN*2^12, (PPN+1)*2^12)
    println!(
        "free physical memory paddr = [{:#x}, {:#x})",
        ((end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) >> 12) + 1,
        PHYSICAL_MEMORY_END >> 12
    );
    crate::interrupt::init();

    // 物理内存初始化
    crate::memory::init(
        ((end as usize - KERNEL_BEGIN_VADDR + KERNEL_BEGIN_PADDR) >> 12) + 1,
        PHYSICAL_MEMORY_END >> 12,
    );
    frame_allocating_test();
    dynamic_allocating_test();

    // 时钟初始化
    crate::timer::init();
    unsafe {
        asm!("ebreak"::::"volatile");
    }
    panic!("end of rust_main");
    loop {}
}

fn frame_allocating_test() {
    println!("alloc {:x?}", alloc_frame());
    let f = alloc_frame();
    println!("alloc {:x?}", f);
    println!("alloc {:x?}", alloc_frame());
    println!("dealloc {:x?}", f);
    dealloc_frame(f.unwrap());
    println!("alloc {:x?}", alloc_frame());
    println!("alloc {:x?}", alloc_frame());
}

fn dynamic_allocating_test() {
    use alloc::boxed::Box;
    use alloc::vec::Vec;

    extern "C" {
        fn sbss();
        fn ebss();
    }
    let lbss = sbss as usize;
    let rbss = ebss as usize;

    let heap_value = Box::new(5);
    assert_eq!(*heap_value, 5);
    println!("heap_value assertion successfully!");
    println!("heap_value is at {:p}", heap_value);
    let heap_value_addr = &*heap_value as *const _ as usize;
    assert!(heap_value_addr >= lbss && heap_value_addr < rbss);
    println!("heap_value is in section .bss!");

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    for i in 0..500 {
        assert_eq!(vec[i], i);
    }
    println!("vec assertion successfully!");
    println!("vec is at {:p}", vec.as_slice());
    let vec_addr = vec.as_ptr() as usize;
    assert!(vec_addr >= lbss && vec_addr < rbss);
    println!("vec is in section .bss!");
}
