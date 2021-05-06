mod frame_allocator;

use crate::consts::*;
use buddy_system_allocator::LockedHeap;

use frame_allocator::SEGMENT_TREE_ALLOCATOR as FRAME_ALLOCATOR;
use riscv::addr::{
    Frame,
    Page,
    PhysAddr,
    // 分别为虚拟地址、物理地址、虚拟页、物理页帧
    // 用法可参见 https://github.com/rcore-os/riscv/blob/master/src/addr.rs
    VirtAddr,
};

pub fn init(l: usize, r: usize) {
    FRAME_ALLOCATOR.lock().init(l, r);
    init_heap();
    println!("+++ setup memory! +++")
}

pub fn alloc_frame() -> Option<Frame> {
    // 将物理页号转为物理页帧
    Some(Frame::of_ppn(FRAME_ALLOCATOR.lock().alloc()))
}

pub fn dealloc_frame(f: Frame) {
    FRAME_ALLOCATOR.lock().dealloc(f.number())
}

fn init_heap() {
    // 同样是在内核中开一块静态内存供 buddy system allocator 使用
    static mut HEAP: [u8; KERNEL_HEAP_SIZE] = [0; KERNEL_HEAP_SIZE];
    unsafe {
        // 这里需要先开锁 才能进行操作
        DYNAMIC_ALLOCATOR
            .lock()
            .init(HEAP.as_ptr() as usize, KERNEL_HEAP_SIZE);
    }
}

#[global_allocator]
static DYNAMIC_ALLOCATOR: LockedHeap = LockedHeap::empty();

#[alloc_error_handler]
fn alloc_error_handler(_: core::alloc::Layout) -> ! {
    panic!("alloc_error_handler do noting but panic!");
}
