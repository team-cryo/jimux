use core::fmt;

use super::matrix::{*, matrix_fixed::*};
use self::color::Color;
use self::style::Style;
use self::symbol::{*, symbol_vga::*};
use self::style::style_vga::StyleVGA;

pub mod color;
pub mod style;
pub mod symbol;
pub mod charset;

pub const ROWS: usize = 25;
pub const COLUMNS: usize = 80;

///
/// A VGA buffer with double buffering support.
/// 
/// ## Initialization
/// ```
/// let mut vga = VGACanvas::new(0xb8000 as *mut u8);
/// ```
///
pub struct VGACanvas
{
    vga_buffer_ptr: *mut u8,
    buffer: MatrixFixed<SymbolVGA, ROWS, COLUMNS>,
    marker: usize
}

impl VGACanvas
{
    //TODO const impl
    pub fn new(vga_buffer_ptr: *mut u8) -> Self
    {
        Self
        {
            vga_buffer_ptr,
            buffer: MatrixFixed::repeat(SymbolVGA::new(' ' as u8, StyleVGA::default())),
            marker: 0
        }
    }

    pub fn fill(&mut self, fill: SymbolVGA)
    {
        self.buffer.fill(fill)
    }

    pub fn fill_frame(&mut self, image: MatrixFixed<SymbolVGA, ROWS, COLUMNS>)
    {
        self.buffer = image;
    }

    pub fn capture(&self) -> MatrixFixed<SymbolVGA, ROWS, COLUMNS>
    {
        self.buffer.clone()
    }

    pub fn wipe(&mut self)
    {
        self.fill_frame(MatrixFixed::new())
    }

    pub fn fill_frame_overlay(&mut self, image: &MatrixFixed<Option<SymbolVGA>, ROWS, COLUMNS>)
    {
        let serial = self.buffer.serial_mut();
        for (i, cell) in image.iter().enumerate()
        {
            match cell
            {
                Some(symbol) => serial[i].overlay(symbol),
                None => ()
            }
        }
    }
    
    pub fn put_image<const R: usize, const C: usize>(&mut self, image: &MatrixFixed<SymbolVGA, R, C>)
    where [SymbolVGA; R*C]: Sized
    {
        self.put_image_overlay(&image.as_op())
    }

    pub fn put_image_overlay<const R: usize, const C: usize>(&mut self, image: &MatrixFixed<Option<SymbolVGA>, R, C>)
    where [Option<SymbolVGA>; R*C]: Sized
    {
        let mpos = self.marker_pos();
        for (r, row) in image.rows().iter().enumerate()
        {
            for (c, s) in row.iter().enumerate()
            {
                match s
                {
                    Some(symbol) => match self.buffer.get_mut(r + mpos.0, c + mpos.1)
                    {
                        Some(cell) => cell.overlay(symbol),
                        None => ()
                    },
                    None => ()
                }
            }
        }
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

    pub fn put_symbol(&mut self, symbol: &SymbolVGA)
    {
        let i: usize = self.marker;
        self.marker += 1;

        assert!(i < ROWS*COLUMNS);

        self.buffer.serial_mut()[i].overlay(symbol);
    }

    pub fn put_plain_text(&mut self, string: &[u8])
    {
        self.put_text(string, StyleVGA::default())
    }

    pub fn put_text<S, C>(&mut self, string: &[u8], style: S)
    where S: ~const Style<C> + Copy, C: Color
    {
        let serial = self.buffer.serial_mut();

        for char in string.iter()
        {
            let i: usize = self.marker;
            self.marker += 1;
            
            match serial.get_mut(i)
            {
                Some(s) => s.overlay(&SymbolVGA::new(*char, style)),
                None => break
            };
        }
    }

    pub fn marker_pos(&self) -> (usize, usize)
    {
        let r = self.marker / COLUMNS;
        let c = self.marker % COLUMNS;
        
        (r, c)
    }

    pub fn at_marker(&self) -> &SymbolVGA
    {
        &self.buffer[self.marker_pos()]
    }

    pub fn at_marker_mut(&mut self) -> &mut SymbolVGA
    {
        let pos = self.marker_pos();
        &mut self.buffer[pos]
    }

    pub fn new_line(&mut self)
    {
        self.set_marker(self.marker_pos().0 + 1, 0);
    }

    pub fn indent(&mut self, spaces: usize)
    {
        self.set_marker(self.marker_pos().0, self.marker_pos().1 + spaces);
    }

    pub fn render(&mut self)
    {
        for (i, &sym) in self.buffer.iter().enumerate() {
            unsafe {
                *self.vga_buffer_ptr.offset(i as isize * 2) = sym.char() as u8;
                *self.vga_buffer_ptr.offset(i as isize * 2 + 1) = sym.style().as_byte();
            }
        }
    }
}

pub fn char_table() -> MatrixFixed<SymbolVGA, 8, 32>
{
    let mut char_table: MatrixFixed<SymbolVGA, 8, 32> = MatrixFixed::new();

    for (i, s) in char_table.serial_mut().iter_mut().enumerate()
    {
        *s = SymbolVGA::new(i as u8, StyleVGA::default());
    }

    return char_table
}

pub struct VGABufferWriter
{
    buffer: VGACanvas
}

impl VGABufferWriter
{
    fn write_line()
    {
        
    }
}

#[macro_export]
macro_rules! print {
    ($($arg: tt)*) => {
        let mut vga = VGACanvas::new(0xb8000 as *mut u8);
        vga.put_text(&format_args!($($arg)*).as_str().unwrap().as_bytes(), StyleVGA::default());
        vga.new_line();
        vga.render();
    }
}