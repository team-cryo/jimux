use crate::number::float::FloatType;

use super::*;

impl Number for u32
{

}

impl Integer for u32
{
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
    const ZERO: Self = 0;
    
    fn from_float<F>(float: F) -> Self
    where F: Float
    {
        match float.as_enum()
        {
            FloatType::F32(f) => f as u32,
            FloatType::F64(f) => f as u32,
        }
    }
}

impl UnsignedInteger for u32
{

}