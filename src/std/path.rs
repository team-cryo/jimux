use super::ffi::OsStr;

//https://doc.rust-lang.org/std/path/struct.Path.html
pub struct Path
{
    inner: OsStr
}

impl Path
{
    pub fn new<S: AsRef<OsStr> + ?Sized>(s: &S) -> &Path
    {
        todo!()
    }

    pub fn as_os_str(&self) -> &OsStr
    {
        &self.inner
    }

    pub fn to_str(&self) -> Option<&str>
    {
        todo!()
    }

    pub fn to_str_lossy(&self) -> Cow<'_, str>
    {
        
    }
}