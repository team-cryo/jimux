use core::ops::{Mul, Add, Sub, Div};

use super::Number;

mod f32;
mod f64;

pub enum FloatType
{
    F32(f32),
    F64(f64)
}

pub trait Float:
    Number +
    Add<Self, Output = Self> +
    Sub<Self, Output = Self> +
    Mul<Self, Output = Self> +
    Div<Self, Output = Self> +
    From<u8> + From<u16> +
    From<i8> + From<i16> +
    From<f32>
{
    fn as_enum(self) -> FloatType;
}