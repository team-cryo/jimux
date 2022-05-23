use crate::number::false_primitive::FalsePrimitive;

use super::*;

#[allow(non_camel_case_types)]
pub struct u2
{
    prim: u8
}

impl Number for u2
{

}

impl Integer for u2
{

}

impl UnsignedInteger for u2
{

}

impl FalsePrimitive<u8> for u2
{
    fn prim(&self) -> u8
    {
        self.prim % (1 << 2)
    }
}

impl From<u8> for u2
{
    fn from(prim: u8) -> Self
    {
        Self
        {
            prim
        }
    }
}

impl Into<u8> for u2
{
    fn into(self) -> u8
    {
        self.prim()
    }
}