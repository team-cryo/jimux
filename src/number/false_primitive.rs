pub trait FalsePrimitive<T>: Sized + From<T> + Into<T>
where T: Sized
{
    fn prim(&self) -> T;
}