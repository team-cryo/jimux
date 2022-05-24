use crate::number::Number;

use super::{Float, FloatType};

impl Number for f32
{

}

impl Float for f32
{
    fn as_enum(self) -> FloatType
    {
        FloatType::F32(self)
    }
}