use core::array::IntoIter;

use crate::number::integer::unsigned::{u1::u1, u2::u2, u4::u4, u6::u6};

use super::{Color, ColorHex};
use super::rgb::ColorRGB;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ColorVGA
{
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15
}

impl ColorVGA
{
    pub fn to_rgb(&self) -> ColorRGB<u2>
    {
        ColorRGB::from_hex(self.hex())
    }

    pub const fn id(self) -> u4
    {
        u4::from(self as u8)
    }

    pub const fn variants() -> [Self; 16]
    {
        [
            Self::Black,
            Self::Blue,
            Self::Green,
            Self::Cyan,
            Self::Red,
            Self::Magenta,
            Self::Brown,
            Self::LightGray,
            Self::DarkGray,
            Self::LightBlue,
            Self::LightGreen,
            Self::LightCyan,
            Self::LightRed,
            Self::Pink,
            Self::Yellow,
            Self::White
        ]
    }

    pub fn iter() -> IntoIter<Self, 16>
    {
        Self::variants().into_iter()
    }
}

impl Default for ColorVGA
{
    fn default() -> Self
    {
        Self::Black
    }
}

impl Color for ColorVGA
{
    fn vga(&self) -> Self
    {
        self.clone()
    }
}

impl ColorHex<u6> for ColorVGA
{
    fn from_hex(hex: u6) -> Self
    {
        ColorRGB::from_hex(hex).vga()
    }
    fn hex(&self) -> u6
    {
        match self
        {
            Self::Black => u6::from(0),
            Self::Blue => u6::from(1),
            Self::Green => u6::from(2),
            Self::Cyan => u6::from(3),
            Self::Red => u6::from(4),
            Self::Magenta => u6::from(5),
            Self::Brown => u6::from(20),
            Self::LightGray => u6::from(7),
            Self::DarkGray => u6::from(56),
            Self::LightBlue => u6::from(57),
            Self::LightGreen => u6::from(58),
            Self::LightCyan => u6::from(59),
            Self::LightRed => u6::from(60),
            Self::Pink => u6::from(61),
            Self::Yellow => u6::from(62),
            Self::White => u6::from(63)
        }
    }
}

impl Into<ColorRGB<u1>> for ColorVGA
{
    fn into(self) -> ColorRGB<u1>
    {
        self.to_rgb().rescale32()
    }
}

impl Into<ColorRGB<u2>> for ColorVGA
{
    fn into(self) -> ColorRGB<u2>
    {
        self.to_rgb()
    }
}

impl Into<ColorRGB<u8>> for ColorVGA
{
    fn into(self) -> ColorRGB<u8>
    {
        self.to_rgb().rescale32()
    }
}