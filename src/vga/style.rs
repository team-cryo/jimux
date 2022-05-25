use self::style_vga::StyleVGA;

use super::color::Color;

pub mod style_vga;

pub trait Style<C>
where C: Color
{
    fn vga(self) -> StyleVGA;
}

pub trait Overlay
{
    fn overlay(&mut self, top: &Self);
}