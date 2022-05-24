use crate::number::false_primitive::FalsePrimitive;
use crate::number::float::FloatType;

use super::*;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct u6(u8);

impl Number for u6
{

}

impl Integer for u6
{
    const MIN: Self = Self::from(0);
    const MAX: Self = Self::from(((1 << 6) - 1) as u8);
    const ZERO: Self = Self::from(0);

    fn from_float<F>(float: F) -> Self
    where F: Float
    {
        match float.as_enum()
        {
            FloatType::F32(f) => f.into(),
            FloatType::F64(f) => f.into(),
        }
    }
}

impl const From<f32> for u6
{
    fn from(float: f32) -> Self
    {
        Self::from(float as u8)
    }
}
impl const From<f64> for u6
{
    fn from(float: f64) -> Self
    {
        Self::from(float as u8)
    }
}
impl Into<f32> for u6
{
    fn into(self) -> f32
    {
        self.prim().into()
    }
}
impl Into<f64> for u6
{
    fn into(self) -> f64
    {
        self.prim().into()
    }
}

impl UnsignedInteger for u6
{

}

impl FalsePrimitive<u8> for u6
{
    fn prim(&self) -> u8
    {
        self.0 % (1 << 6)
    }
}

impl const From<u8> for u6
{
    fn from(prim: u8) -> Self
    {
        Self(prim)
    }
}

impl Into<u8> for u6
{
    fn into(self) -> u8
    {
        self.prim()
    }
}