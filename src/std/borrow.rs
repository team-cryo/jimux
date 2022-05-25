use core::hash::{Hasher, Hash};
use core::ops::AddAssign;

pub enum Cow<'a, B>
where
    B: 'a + ToOwned + ?Sized
{
    Borrowed(&'a B),
    ToOwned(<B as ToOwned>::Owned)
}

//FINISHED:

pub trait ToOwned
{
    type Owned: Borrow<Self>;

    fn to_owned(&self) -> Self::Owned
    {
        self.clone()
    }

    fn clone_into(&self, target: &mut Self::Owned)
    {
        target.clone_from(self)
    } 
}

pub trait Borrow<Borrowed>
where Borrowed: ?Sized
{
    fn borrow(&self) -> &Borrowed;
}

impl<B: ?Sized> Hash for Cow<'_, B>
where
    B: Hash + ToOwned,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        Hash::hash(&**self, state)
    }
}

impl<T: ?Sized + ToOwned> AsRef<T> for Cow<'_, T>
{
    fn as_ref(&self) -> &T
    {
        self
    }
}

impl<T: ?Sized + Hash> Hash for &T
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        (**self).hash(state);
    }
}

impl<T: ?Sized + Hash> Hash for &mut T
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        (**self).hash(state);
    }
}

impl<T: ?Sized> Hash for *const T
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        let (address, metadata) = self.to_raw_parts();
        state.write_usize(address.addr());
        metadata.hash(state);
    }
}

impl<T: ?Sized> Hash for *mut T
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H)
    {
        let (address, metadata) = self.to_raw_parts();

        state.write_usize(address.addr());
        metadata.hash(state);
    }
}