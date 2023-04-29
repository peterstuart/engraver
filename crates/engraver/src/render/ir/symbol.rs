use super::{Convert, Coord};

#[derive(Clone, Debug)]
pub struct Symbol<T> {
    pub origin: Coord<T>,
    pub value: char,
}

impl<T> Symbol<T> {
    pub fn convert<U>(self, converter: &impl Convert<T, U>) -> Symbol<U> {
        Symbol {
            origin: self.origin.convert(converter),
            value: self.value,
        }
    }

    pub fn max_x(&self) -> T
    where
        T: Copy,
    {
        self.origin.x
    }
}
