use crate::vga::color::color_vga::ColorVGA;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymbolVGA
{
    char: u8,
    color: ColorVGA
}

impl SymbolVGA
{
    pub fn new<C>(char: u8, color: C) -> Self
    where C: Color
    {
        Self
        {
            char: char,
            color: color.vga()
        }
    }
}

impl Symbol<ColorVGA> for SymbolVGA
{
    fn char(&self) -> u8
    {
        self.char
    }

    fn color(&self) -> ColorVGA
    {
        self.color
    }
}

impl Default for SymbolVGA
{
    fn default() -> Self
    {
        Self::new(' ' as u8, ColorVGA::default())
    }
}