use crate::number::false_primitive::FalsePrimitive;
use crate::number::integer::unsigned::u3::u3;
use crate::vga::color::ColorHex;
use crate::vga::color::rgb::ColorRGB;
use crate::vga::color::vga::ColorVGA;
use crate::number::integer::unsigned::u1::u1;

use super::Style;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StyleVGA
{
    data: u8,
    opaque: bool
}

impl StyleVGA
{
    pub fn new(color: ColorVGA, background: Option<ColorRGB<u1>>, blink: bool) -> Self
    {
        let mut data = color as u8;
        let mut opaque = true;

        match background
        {
            Some(bg) => data += bg.hex().prim() << 4,
            None => opaque = false
        }

        if blink
        {
            data += 1 << 7;
        }

        Self
        {
            data,
            opaque
        }
    }

    pub fn as_byte(&self) -> u8
    {
        self.data
    }

    pub fn blink(&self) -> bool
    {
        self.data >> 7 != 0
    }

    pub fn opaque(&self) -> bool
    {
        self.opaque
    }

    pub fn color(&self) -> ColorVGA
    {
        ColorVGA::variants()[(self.data % 16) as usize]
    }

    pub fn background(&self) -> Option<ColorRGB<u1>>
    {
        match self.opaque
        {
            true => Some(ColorRGB::from_hex(u3::from(self.data >> 4))),
            false => None
        }
    }
}

impl From<ColorVGA> for StyleVGA
{
    fn from(color: ColorVGA) -> Self
    {
        Self
        {
            data: color as u8,
            opaque: true
        }
    }
}

impl Style<ColorVGA> for StyleVGA
{
    fn vga(self) -> Self
    {
        self
    }

    fn overlay(&mut self, top: &Self)
    {
        *self = Self::new(top.color(), top.background().or(self.background()), top.blink())
    }
}

impl Default for StyleVGA
{
    fn default() -> Self
    {
        Self::new(ColorVGA::White, None, false)
    }
}