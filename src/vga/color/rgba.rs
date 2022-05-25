use super::{*, rgb::ColorRGB};
use crate::number::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorRGBA<T>
where T: Number
{
    color: ColorRGB<T>,
    a: T
}

impl<T> ColorRGBA<T>
where ColorRGB<T>: Color, T: Number
{
    pub const fn new(r: T, g: T, b: T, a: T) -> Self
    {
        Self
        {
            color: ColorRGB {
                r,
                g,
                b
            },
            a: a
        }
    }
}

//TODO const impl
impl<T> Color for ColorRGBA<T>
where ColorRGB<T>: Color, T: Number
{
    fn vga(&self) -> ColorVGA
    {
        self.color.vga()
    }
}