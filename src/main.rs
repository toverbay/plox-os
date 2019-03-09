#![no_std]  // kernel can't depend on platform libraries.
#![no_main] // kernel isn't using the normal main() entry point.
#![feature(asm)] // enables the asm! macro for inline assembly

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

// Linux-style entry point
// #[cfg(target_os = "linux")]
#[no_mangle] // Compiler typically mangles symbols. We don't want that here.
pub extern "C" fn _start() -> ! { //..external function with C calling convention.
    // This function is the entry point, since the linker looks for a function
    // named '_start' by default
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// #[cfg(target_os = "windows")]
// #[no_mangle] // Compiler typically mangles symbols. We don't want that here.
// pub extern "C" fn mainCRTStartup() -> ! {
//     main();
// }
// #[cfg(not(target_os = "linux"))]
// #[no_mangle] // Compiler typically mangles symbols. We don't want that here.
// pub extern "C" fn main() -> ! {
//     loop {}
// }

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
