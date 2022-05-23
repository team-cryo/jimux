use super::color::Color;

pub mod symbol_vga;

pub trait Symbol<C>
where C: Color
{
    fn char(&self) -> u8;
    fn color(&self) -> C;
}