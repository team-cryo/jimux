use crate::number::integer::unsigned_integer::UnsignedInteger;

use self::color_vga::ColorVGA;

pub mod color_vga;
pub mod color_rgb;
pub mod color_rgba;

pub trait Color
{
    fn vga(&self) -> ColorVGA;
}

pub trait ColorHex<I>
where I: UnsignedInteger
{
    fn hex(&self) -> I;
    fn from_hex(hex: I) -> Self;
}