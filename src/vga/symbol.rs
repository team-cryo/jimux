use super::color::Color;
use super::style::Style;

pub mod symbol_vga;

pub trait Symbol<S, C>
where S: Style<C>, C: Color
{
    fn char(&self) -> u8;
    fn style(&self) -> S;
    fn overlay(&mut self, top: &Self);
}