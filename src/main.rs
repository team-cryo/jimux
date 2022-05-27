#![no_std]
#![no_main]

#![feature(iter_collect_into)]
#![feature(generic_const_exprs)] //causes occational rls crashes, but very convenient
#![feature(associated_const_equality)]
#![feature(const_trait_impl)]
#![feature(slice_as_chunks)]
#![feature(split_array)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![feature(panic_info_message)]
#![feature(mixed_integer_ops)]
#![feature(never_type)]

#![allow(dead_code)]

use core::panic::PanicInfo;

use self::matrix::matrix_fixed::MatrixFixed;
use self::vga::VGACanvas;
use self::vga::charset::CharsetVGA;
use self::vga::color::vga::ColorVGA;
use self::vga::style::style_vga::StyleVGA;
use self::vga::symbol::symbol_vga::SymbolVGA;

//mod std;
mod number;
mod vga;
mod matrix;
mod math;

const HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> !
{
    print!("Hello World");

    let mut vga = VGACanvas::new(0xb8000 as *mut u8);

    vga.fill(SymbolVGA::new('r' as u8, ColorVGA::Red));
    
    vga.fill(SymbolVGA::new(CharsetVGA::TileLight as u8, ColorVGA::LightBlue));
    
    let square: MatrixFixed<SymbolVGA, {vga::ROWS - 3}, {vga::COLUMNS - 2}> = MatrixFixed::repeat(
        SymbolVGA::new(CharsetVGA::FullBlock as u8, StyleVGA::new(ColorVGA::LightGray, Some(ColorVGA::Pink.into()), false))
    );

    vga.set_marker(2, 1);
    vga.put_image(&square);

    let toolbar_style: StyleVGA = StyleVGA::new(ColorVGA::White, Some(ColorVGA::DarkGray.into()), true);
    let toolbar: MatrixFixed<SymbolVGA, 1, {vga::COLUMNS}> = MatrixFixed::repeat(
        SymbolVGA::new(' ' as u8, toolbar_style)
    );
    
    vga.set_marker(0, 0);
    vga.put_image(&toolbar);

    vga.set_marker(0, 0);
    vga.put_text(b" ", toolbar_style.with_color(ColorVGA::DarkGray)); //removes blinking underline
    vga.put_text(b"jimux 1.0", toolbar_style);

    vga.set_marker(21, 2);
    vga.put_text(&HELLO, ColorVGA::White);
    vga.new_line();
    vga.indent(2);
    vga.put_text(&HELLO, ColorVGA::White);
    vga.render();
    vga.put_plain_text(&HELLO);

    let mut square: MatrixFixed<SymbolVGA, 10, 10> = MatrixFixed::repeat(
        SymbolVGA::new('~' as u8, StyleVGA::new(ColorVGA::Yellow, Some(ColorVGA::White.into()), false))
    );
    square[(1, 1)] = SymbolVGA::new(CharsetVGA::Smiley as u8, StyleVGA::new(ColorVGA::Yellow, Some(ColorVGA::Cyan.into()), false));

    vga.set_marker(10, 50);
    vga.put_image(&square);

    vga.set_marker(12, 20);
    vga.put_text(&HELLO, StyleVGA::new(ColorVGA::Blue, None, true));
    vga.new_line();
    vga.indent(20);
    vga.put_text(&HELLO, StyleVGA::new(ColorVGA::LightRed, None, true));

    vga.set_marker(3, 2);
    vga.put_image(&vga::char_table());

    vga.set_marker(3, vga::COLUMNS - 3);
    vga.put_symbol(&SymbolVGA::new(CharsetVGA::AE as u8, StyleVGA::default()));

    vga.render();

    //dette er noe jeg har gjort for å demonstrere canvas, kan bare fjernes om loopen skal brukes til noe annet
    let still_image = vga.capture();
    let mut pos: (usize, usize) = (vga::ROWS/2, vga::COLUMNS/2);
    let mut v: (isize, isize) = (1, 1);
    let bounce = b"<dvd>";
    let mut frame: u128 = 0;
    const FRAME_SKIP: u128 = 32;
    let colors = ColorVGA::variants();
    
    loop
    {
        if frame % FRAME_SKIP == 0
        {
            vga.fill_frame(still_image.clone());
            vga.set_marker(pos.0, pos.1);
            vga.put_text(bounce, colors[(frame/FRAME_SKIP % 16) as usize]);

            if pos.0 <= 0
            {
                v.0 = 1;
            }
            else if pos.0 + 1 >= vga::ROWS
            {
                v.0 = -1;
            }
            if pos.1 <= 0
            {
                v.1 = 1
            }
            else if pos.1 + bounce.len() >= vga::COLUMNS
            {
                v.1 = -1
            }
            pos.0 = (pos.0 as isize + v.0) as usize;
            pos.1 = (pos.1 as isize + v.1) as usize;
        }

        vga.render();
        frame += 1;
    }
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

    vga.put_plain_text(b"Panic!: ");

    match _info.message()
    {
        Some(message) => match message.as_str()
        {
            Some(str) => vga.put_plain_text(str.as_bytes()),
            None => vga.put_plain_text(b"invalid message")
        },
        None => vga.put_plain_text(b"no message")
    }

    vga.render();

    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
}
