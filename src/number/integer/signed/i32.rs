use crate::number::float::FloatType;

use super::*;

impl Number for i32
{

}

impl Integer for i32
{
    const MIN: Self = Self::MIN;
    const MAX: Self = Self::MAX;
    const ZERO: Self = 0;

    fn from_float<F>(float: F) -> Self
    where F: Float
    {
        match float.as_enum()
        {
            FloatType::F32(f) => f as Self,
            FloatType::F64(f) => f as Self,
        }
    }
}

impl SignedInteger for i32
{

}