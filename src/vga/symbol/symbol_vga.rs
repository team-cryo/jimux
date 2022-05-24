use crate::vga::color::vga::ColorVGA;
use crate::vga::style::style_vga::StyleVGA;

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SymbolVGA
{
    char: u8,
    style: StyleVGA
}

impl SymbolVGA
{
    pub fn new<S, C>(char: u8, style: S) -> Self
    where S: Style<C>, C: Color
    {
        Self
        {
            char: char,
            style: style.vga()
        }
    }
}

impl Symbol<StyleVGA, ColorVGA> for SymbolVGA
{
    fn char(&self) -> u8
    {
        self.char
    }

    fn style(&self) -> StyleVGA
    {
        self.style
    }

    fn overlay(&mut self, top: &Self)
    {
        self.char = top.char;
        self.style.overlay(&top.style)
    }
}

impl Default for SymbolVGA
{
    fn default() -> Self
    {
        Self::new(' ' as u8, StyleVGA::default())
    }
}