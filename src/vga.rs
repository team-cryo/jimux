use core::alloc::LayoutError;
use core::fmt;
use core::ops::Index;

use super::matrix::{*, matrix_fixed::*};
use self::color::color_vga::ColorVGA;
use self::symbol::{*, symbol_vga::*};

pub mod color;
pub mod symbol;

pub const ROWS: usize = 25;
pub const COLUMNS: usize = 80;

/**
 * A VGABuffer with double buffering support.
 */

pub struct VGABuffer
{
    vga_buffer_ptr: *mut u8,
    buffer: MatrixFixed<SymbolVGA, ROWS, COLUMNS>,
    marker: usize
}

impl VGABuffer
{
    pub fn new(vga_buffer_ptr: *mut u8) -> Self
    {
        Self
        {
            vga_buffer_ptr,
            buffer: MatrixFixed::repeat(SymbolVGA::new(' ' as u8, ColorVGA::Black)),
            marker: 0
        }
    }

    pub fn fill(&mut self, fill: SymbolVGA)
    {
        self.buffer.fill(fill)
    }

    pub fn fill_image(&mut self, image: MatrixFixed<SymbolVGA, ROWS, COLUMNS>)
    {
        self.buffer = image;
    }

    pub fn wipe(&mut self)
    {
        self.fill_image(MatrixFixed::new())
    }

    pub fn set_marker_serial(&mut self, marker: usize)
    {
        assert!(marker < ROWS*COLUMNS);
        self.marker = marker;
    }

    pub fn set_marker(&mut self, r: usize, c: usize)
    {
        self.set_marker_serial(r*COLUMNS + c);
    }

    pub fn reset_marker(&mut self)
    {
        self.marker = 0;
    }

    pub fn reset(&mut self)
    {
        self.reset_marker();
        self.wipe();
    }

    pub fn put_symbol(&mut self, symbol: SymbolVGA)
    {
        let i: usize = self.marker;
        self.marker += 1;

        assert!(i < ROWS*COLUMNS);
        self.buffer.serial_mut()[i] = symbol;
    }

    pub fn put_char(&mut self, char: u8, color: ColorVGA)
    {
        self.put_symbol(SymbolVGA::new(char, color))
    }

    pub fn put_text(&mut self, string: &[u8], color: ColorVGA)
    {
        let serial = self.buffer.serial_mut();

        for char in string.iter()
        {
            let i: usize = self.marker;
            self.marker += 1;
            
            match serial.get_mut(i)
            {
                Some(s) => *s = SymbolVGA::new(*char, color),
                None => break
            };
        }
    }

    pub fn marker_pos(&self) -> (usize, usize)
    {
        (self.marker / COLUMNS, self.marker % COLUMNS)
    }

    pub fn new_line(&mut self)
    {
        self.set_marker(self.marker_pos().0 + 1, 0);
    }

    pub fn render(&mut self)
    {
        for (i, &sym) in self.buffer.iter().enumerate() {
            unsafe {
                *self.vga_buffer_ptr.offset(i as isize * 2) = sym.char() as u8;
                *self.vga_buffer_ptr.offset(i as isize * 2 + 1) = sym.color() as u8;
            }
        }
    }
}

pub struct VGABufferWriter
{
    buffer: VGABuffer
}

impl VGABufferWriter
{
    fn write_line() {
        
    }
}

#[macro_export]
macro_rules! print {
    ($($arg: tt)*) => {
        vga::_print(format_args!($($arg)*));
    }
}

pub fn _print(args: fmt::Arguments) {
    let mut vga_buffer = VGABuffer::new(0xb8000 as *mut u8);
    vga_buffer.put_text(&args.as_str().unwrap().as_bytes(), ColorVGA::White);
    vga_buffer.new_line();
    vga_buffer.render();
}
