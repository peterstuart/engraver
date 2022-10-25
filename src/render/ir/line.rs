use super::{Convert, Coord};
use crate::render::math;

#[derive(Clone, Copy, Debug)]
pub enum Linecap {
    Butt,
    Round,
}

#[derive(Clone, Copy, Debug)]
pub struct Line<T> {
    pub from: Coord<T>,
    pub to: Coord<T>,
    pub thickness: T,
    pub cap: Linecap,
}

impl<T> Line<T> {
    pub fn convert<U>(self, converter: &impl Convert<T, U>) -> Line<U> {
        Line {
            from: self.from.convert(converter),
            to: self.to.convert(converter),
            thickness: converter.convert_thickness(self.thickness),
            cap: self.cap,
        }
    }

    pub fn max_x(&self) -> T
    where
        T: Copy + PartialOrd,
    {
        math::max(self.from.x, self.to.x)
    }
}
