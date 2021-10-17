use std::alloc::{GlobalAlloc, Layout, System};
struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let data = System.alloc(layout);
        eprintln!("ALLOC: {:p}, size {}", data, layout.size());
        data
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        eprintln!("DEALLOC: {:p}, size {}", ptr, layout.size());
    }
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;

#[allow(dead_code)]
struct Matrix {
    data: [u8; 505],
}

impl Default for Matrix {
    fn default() -> Self {
        Self { data: [0; 505] }
    }
}

 #[test]
 fn test_allocator() {
     let data = Box::new(Matrix::default());
     println!(
         "!!! allocated memory: {:p}, len: {}", 
         &*data,
         std::mem::size_of::<Matrix>()
     );
 }

// 分配 16M 的内存，在 Debug 模式下，这个测试会失败
// 在 release 模式下，才能成功
// #[test]
// fn test_alloc_16m() {
//     let boxed = Box::new([0u8; 1<<24]);
//     println!("len: {}", boxed.len());
// }