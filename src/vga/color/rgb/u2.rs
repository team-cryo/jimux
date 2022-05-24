use crate::number::false_primitive::FalsePrimitive;
use crate::number::integer::unsigned::{u2::u2, u6::u6};
use crate::vga::color::{Color, ColorHex};
use crate::vga::color::vga::ColorVGA;

use super::ColorRGB;

impl Color for ColorRGB<u2>
{
    fn vga(&self) -> ColorVGA
    {
        let mut select: ColorVGA = ColorVGA::Black;
        let mut ds: u16 = u16::MAX; //only 10 bits used here
        for c0 in ColorVGA::iter()
        {
            let c = c0.to_rgb();
            let dr: u16 = (c.r.prim() as i8 - self.r.prim() as i8).abs() as u16;
            let dg: u16 = (c.g.prim() as i8 - self.g.prim() as i8).abs() as u16;
            let db: u16 = (c.b.prim() as i8 - self.b.prim() as i8).abs() as u16;
            let d: u16 = dr*dr + dg*dg + db*db;
            if d < ds
            {
                select = c0;
                ds = d;
            }
        }
        select
    }
}

impl ColorHex<u6> for ColorRGB<u2>
{
    fn hex(&self) -> u6
    {
        u6::from(self.r.prim() << 4 + self.g.prim() << 2 + self.b.prim())
    }

    fn from_hex(hex: u6) -> Self
    {
        Self
        {
            r: u2::from(hex.prim() >> 4),
            g: u2::from((hex.prim() % (1 << 4)) >> 2),
            b: u2::from(hex.prim() % (1 << 2))
        }
    }
}