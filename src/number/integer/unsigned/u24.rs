use crate::number::false_primitive::FalsePrimitive;
use crate::number::float::FloatType;

use super::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct u24(u32);

impl Number for u24
{
    
}

impl Integer for u24
{
    const MIN: Self = Self::from(0);
    const MAX: Self = Self::from(((1 << 24) - 1) as u32);
    const ZERO: Self = Self::from(0);
    
    fn from_float<F>(float: F) -> Self
    where F: Float
    {
        match float.as_enum()
        {
            FloatType::F32(f) => f.into(), //inaccurate conversion
            FloatType::F64(f) => f.into(),
        }
    }
}

impl const From<f64> for u24
{
    fn from(float: f64) -> Self
    {
        Self::from(float as u32)
    }
}
impl const From<f32> for u24
{
    fn from(float: f32) -> Self
    {
        Self::from(float as u32)
    }
}
impl Into<f64> for u24
{
    fn into(self) -> f64
    {
        self.prim().into()
    }
}

impl UnsignedInteger for u24
{

}

impl FalsePrimitive<u32> for u24
{
    fn prim(&self) -> u32
    {
        self.0 % (1 << 24)
    }
}

impl const From<u32> for u24
{
    fn from(prim: u32) -> Self
    {
        Self(prim)
    }
}
impl Into<u32> for u24
{
    fn into(self) -> u32
    {
        self.prim()
    }
}