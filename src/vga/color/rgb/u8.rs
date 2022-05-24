use crate::number::false_primitive::FalsePrimitive;
use crate::number::integer::unsigned::{u2::u2, u24::u24};
use crate::vga::color::{Color, ColorHex};
use crate::vga::color::vga::ColorVGA;

use super::{ColorRGB};

impl Color for ColorRGB<u8>
{
    fn vga(&self) -> ColorVGA
    {
        self.rescale32::<u2>().vga()
    }
}

impl ColorHex<u24> for ColorRGB<u8>
{
    fn hex(&self) -> u24
    {
        u24::from((self.r as u32) << 16 + (self.g as u32) << 8 + (self.b as u32))
    }

    fn from_hex(hex: u24) -> Self
    {
        Self
        {
            r: (hex.prim() >> 16) as u8,
            g: ((hex.prim() % (1 << 16)) >> 8) as u8,
            b: (hex.prim() % (1 << 8)) as u8
        }
    }
}