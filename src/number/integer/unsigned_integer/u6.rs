use crate::number::false_primitive::FalsePrimitive;

use super::*;

#[allow(non_camel_case_types)]
pub struct u6
{
    prim: u8
}

impl Number for u6
{

}

impl Integer for u6
{

}

impl UnsignedInteger for u6
{

}

impl FalsePrimitive<u8> for u6
{
    fn prim(&self) -> u8
    {
        self.prim % (1 << 6)
    }
}

impl From<u8> for u6
{
    fn from(prim: u8) -> Self
    {
        Self
        {
            prim
        }
    }
}

impl Into<u8> for u6
{
    fn into(self) -> u8
    {
        self.prim()
    }
}