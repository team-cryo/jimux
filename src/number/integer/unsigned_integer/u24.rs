use crate::number::false_primitive::FalsePrimitive;

use super::*;

#[allow(non_camel_case_types)]
pub struct u24
{
    prim: u32
}

impl Number for u24
{

}

impl Integer for u24
{

}

impl UnsignedInteger for u24
{

}

impl FalsePrimitive<u32> for u24
{
    fn prim(&self) -> u32
    {
        self.prim % (1 << 24)
    }
}

impl From<u32> for u24
{
    fn from(prim: u32) -> Self
    {
        Self
        {
            prim
        }
    }
}

impl Into<u32> for u24
{
    fn into(self) -> u32
    {
        self.prim()
    }
}