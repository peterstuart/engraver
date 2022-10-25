use std::ops::{Add, Sub};

use super::{Convert, Coord, Size};
use crate::render::math;

#[derive(Clone, Copy, Debug)]
pub struct Rect<T> {
    pub origin: Coord<T>,
    pub size: Size<T>,
}

impl<T> Rect<T> {
    pub fn convert<U>(self, converter: &impl Convert<T, U>) -> Rect<U>
    where
        T: Add<Output = T> + Copy,
        U: Copy + PartialOrd + Sub<Output = U>,
    {
        let from = self.origin.convert(converter);
        let to = (self.origin + self.size).convert(converter);

        let origin = Coord {
            x: math::min(from.x, to.x),
            y: math::min(from.y, to.y),
        };

        let size = Size {
            width: math::difference(to.x, from.x),
            height: math::difference(to.y, from.y),
        };

        Rect { origin, size }
    }

    pub fn max_x(&self) -> T
    where
        T: Add<Output = T> + Copy,
    {
        self.origin.x + self.size.width
    }
}
