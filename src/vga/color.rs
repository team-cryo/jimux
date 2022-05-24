use crate::number::integer::unsigned::UnsignedInteger;

use self::vga::ColorVGA;

pub mod vga;
pub mod rgb;
pub mod rgba;

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