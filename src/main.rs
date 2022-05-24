#![no_std]
#![no_main]

#![feature(iter_collect_into)]
#![feature(generic_const_exprs)] //causes occational rls crashes, but very convenient
#![feature(associated_const_equality)]
#![feature(const_trait_impl)]
#![feature(slice_as_chunks)]
#![feature(split_array)]
<<<<<<< Updated upstream
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
=======
#![feature(panic_info_message)]
#![feature(mixed_integer_ops)]

>>>>>>> Stashed changes
#![allow(dead_code)]

use core::panic::PanicInfo;

use alloc::string::ToString;

use self::matrix::Matrix;
use self::matrix::matrix_fixed::MatrixFixed;
use self::number::integer::unsigned::u1::u1;
use self::vga::VGACanvas;
use self::vga::charset::Charset;
//use self::vga::charset::Charset;
use self::vga::color::rgb::ColorRGB;
use self::vga::color::vga::ColorVGA;
use self::vga::style::style_vga::StyleVGA;
use self::vga::symbol::symbol_vga::SymbolVGA;

mod std;
mod number;
mod vga;
mod matrix;
mod math;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> !
{
    let mut vga = VGACanvas::new(0xb8000 as *mut u8);

<<<<<<< Updated upstream
    // vga::print("Hello World");
    print!("Hello World");

    // vga_buffer.fill(SymbolVGA::new('r' as u8, ColorVGA::Red));
=======
    let background_color: ColorRGB<u1> = ColorVGA::Red.into();
>>>>>>> Stashed changes

    vga.fill(SymbolVGA::new(39, StyleVGA::new(ColorVGA::Red, Some(background_color), true)));

<<<<<<< Updated upstream
    // vga_buffer.set_marker(20, 10);
    // vga_buffer.put_text(&HELLO, ColorVGA::White);
    // vga_buffer.new_line();
    // vga_buffer.put_text(&HELLO, ColorVGA::White);
    // vga_buffer.render();
=======
    vga.put_plain_text(&HELLO);

    let mut square: MatrixFixed<SymbolVGA, 10, 40> = MatrixFixed::repeat(
        SymbolVGA::new('~' as u8, StyleVGA::new(ColorVGA::Yellow, Some(ColorVGA::White.into()), false))
    );

    let cross: SymbolVGA = SymbolVGA::new('x' as u8, StyleVGA::new(ColorVGA::Yellow, Some(ColorVGA::Cyan.into()), false));

    square[(1, 1)] = cross;

    vga.set_marker(0, 0);
    vga.put_symbol(&cross);

    vga.set_marker(10, 5);
    //assert_eq!(vga.marker_pos(), (10, 0));

    vga.put_image(&square);

    vga.set_marker(10, 20);
    vga.put_text(&HELLO, StyleVGA::new(ColorVGA::Blue, None, true));
    vga.new_line();
    vga.put_text(&HELLO, StyleVGA::new(ColorVGA::LightRed, None, true));

    vga.set_marker(0, 0);
    vga.put_image(&vga::char_table());

    vga.set_marker(23, 0);
    vga.put_symbol(&SymbolVGA::new(Charset::AE as u8, StyleVGA::default()));

    vga.render();
>>>>>>> Stashed changes

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
    let mut vga = VGACanvas::new(0xb8000 as *mut u8);
    vga.set_marker(0, 0);
    vga.put_plain_text(_info.message().into());
    vga.render();

    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}
