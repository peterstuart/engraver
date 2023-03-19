use std::ops::Add;

use super::{Convert, Size};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Coord<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coord<T> {
    pub fn convert<U>(self, converter: &impl Convert<T, U>) -> Coord<U> {
        Coord {
            x: converter.convert_x(self.x),
            y: converter.convert_y(self.y),
        }
    }
}

impl<T> Add<Size<T>> for Coord<T>
where
    T: Add<Output = T>,
{
    type Output = Coord<T>;

    fn add(self, rhs: Size<T>) -> Self {
        Self {
            x: self.x + rhs.width,
            y: self.y + rhs.height,
        }
    }
}
