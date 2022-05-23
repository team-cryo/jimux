#![no_std]
#![no_main]
#![feature(generic_const_exprs)]
#![feature(slice_as_chunks)]
#![feature(split_array)]
#![allow(dead_code)]

use core::panic::PanicInfo;

use self::vga::VGABuffer;
use self::vga::color::color_vga::ColorVGA;
use self::vga::symbol::symbol_vga::SymbolVGA;

mod number;
mod vga;
mod matrix;
mod math;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut vga_buffer = VGABuffer::new(0xb8000 as *mut u8);

    // vga_buffer.fill(SymbolVGA::new('r' as u8, ColorVGA::Red));

    // vga_buffer.put_text(&HELLO, ColorVGA::White);

    // vga_buffer.set_marker(20, 10);
    vga_buffer.put_text(&HELLO, ColorVGA::Blue);
    vga_buffer.new_line();
    vga_buffer.put_text(&HELLO, ColorVGA::Blue);

    for _ in 1..100 {
        vga_buffer.put_text(&HELLO, ColorVGA::Blue);
        vga_buffer.new_line();
        vga_buffer.put_text(&HELLO, ColorVGA::Blue);
    }

    vga_buffer.draw();

    loop {}
}

fn hello_world(vga_buffer: *mut u8)
{
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
}

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    loop {}
}