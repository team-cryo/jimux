use crate::number::float::Float;
use crate::number::integer::Integer;

use super::Color;

mod u2;
mod u1;
mod u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorRGB<T>
{
    pub r: T,
    pub g: T,
    pub b: T
}

impl<T> ColorRGB<T>
where Self: Color
{
    pub fn new(r: T, g: T, b: T) -> Self
    {
        Self {
            r,
            g,
            b
        }
    }

    pub fn rescale<R, F>(self) -> ColorRGB<R>
    where
        ColorRGB<R>: Color,
        R: Integer + Into<F>,
        T: Integer + Into<F>,
        F: Float
    {
        ColorRGB::<R>::new(
            self.r.rescale::<R, F>(),
            self.g.rescale::<R, F>(),
            self.b.rescale::<R, F>()
        )
    }
    
    pub fn rescale32<R>(self) -> ColorRGB<R>
    where
        ColorRGB<R>: Color,
        R: Integer + Into<f32>,
        T: Integer + Into<f32>
    {
        self.rescale::<R, f32>()
    }
    
    pub fn rescale64<R>(self) -> ColorRGB<R>
    where
        ColorRGB<R>: Color,
        R: Integer + Into<f64>,
        T: Integer + Into<f64>
    {
        self.rescale::<R, f64>()
    }
}