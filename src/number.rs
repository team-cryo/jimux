use core::fmt::Debug;

pub mod float;
pub mod integer;
pub mod false_primitive;

pub trait Number: Sized + Copy + Debug + Clone + Copy + PartialEq
{

}