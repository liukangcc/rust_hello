#![no_std]
use core::panic::PanicInfo;

// 导入的 rt-thread 函数列表
extern "C" {
    pub fn rt_kprintf(format: *const u8, ...);
}

#[no_mangle]
pub extern "C"  fn rust_hello()  {
    unsafe {
        rt_kprintf(b"this is from rust\n\0" as *const u8);
        rt_kprintf(b"hello, world\n\0" as *const u8);
        rt_kprintf(b"hello, rt-thread\n\0" as *const u8);
    }
}

#[panic_handler]
fn panic(_info:&PanicInfo) -> !{
    loop{}
}
