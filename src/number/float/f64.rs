use crate::number::Number;

use super::{Float, FloatType};

impl Number for f64
{

}

impl Float for f64
{
    fn as_enum(self) -> FloatType
    {
        FloatType::F64(self)
    }
}