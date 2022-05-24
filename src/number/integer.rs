use super::Number;
use super::float::Float;

pub mod signed;
pub mod unsigned;

pub trait Integer: Number + Eq
{
    const MIN: Self;
    const MAX: Self;
    const ZERO: Self;

    fn rescale<R, F>(self) -> R
    where
        Self: Into<F>,
        R: Integer + Into<F>,
        F: Float
    {
        let b0: (F, F) = (Self::MIN.into(), Self::MIN.into());
        let b1: (F, F) = (R::MIN.into(), R::MIN.into());

        R::from_float((self.into() - b0.0)*(b1.1 - b1.0)/(b0.1 - b0.0) + b1.0)
    }

    fn rescale32<R>(self) -> R
    where
        Self: From<f32> + Into<f32>,
        R: Integer + From<f32> + Into<f32>
    {
        self.rescale::<R, f32>()
    }

    fn rescale64<R>(self) -> R
    where
        Self: From<f64> + Into<f64>,
        R: Integer + From<f64> + Into<f64>
    {
        self.rescale::<R, f64>()
    }

    #[deprecated(since="24.05.2022", note="should be implemented more elegantly after std library is ready")]
    fn from_float<F>(float: F) -> Self
    where F: Float;
}