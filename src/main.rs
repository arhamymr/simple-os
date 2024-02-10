#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;
use vga_buffer::{clear_screen, print_string, write_char, write_something};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

fn write(word: &[u8]) {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in word.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(330 + (i as isize * 2)) = byte;
            *vga_buffer.offset(331 + (i as isize * 2 + 1)) = 0xb;
        }
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    print_string("\n\n\n Hello World!");
    write_something();
    loop {}
}
