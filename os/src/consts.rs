pub const PHYSICAL_MEMORY_END: usize = 0x88000000;
pub const KERNEL_BEGIN_PADDR: usize = 0x80200000;
pub const KERNEL_BEGIN_VADDR: usize = 0x80200000;
pub const MAX_PHYSICAL_MEMEORY: usize = 0x8000000;
pub const MAX_PHYSICAL_PAGES: usize = MAX_PHYSICAL_MEMEORY >> 12;

// 内核堆大小为 8MiB
pub const KERNEL_HEAP_SIZE: usize = 0x800000;
