use core::ops::{Mul, Add, Sub, Div};

use super::Number;

mod f32;
mod f64;

pub trait Float:
    Number +
    Add<Self, Output = Self> +
    Sub<Self, Output = Self> +
    Mul<Self, Output = Self> +
    Div<Self, Output = Self>
{
    
}