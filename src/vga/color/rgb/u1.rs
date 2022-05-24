use crate::number::false_primitive::FalsePrimitive;
use crate::number::integer::unsigned::{u1::u1, u2::u2, u3::u3};
use crate::vga::color::{Color, ColorHex};
use crate::vga::color::vga::ColorVGA;

use super::ColorRGB;

impl Color for ColorRGB<u1>
{
    fn vga(&self) -> ColorVGA
    {
        self.rescale32::<u2>().vga()
    }
}

impl ColorHex<u3> for ColorRGB<u1>
{
    fn hex(&self) -> u3
    {
        u3::from(self.r.prim() << 2 + self.g.prim() << 1 + self.b.prim())
    }

    fn from_hex(hex: u3) -> Self
    {
        Self
        {
            r: u1::from(hex.prim() >> 2),
            g: u1::from(hex.prim() >> 1),
            b: u1::from(hex.prim())
        }
    }
}