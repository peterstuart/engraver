use super::{Convert, Coord};

#[derive(Clone, Debug)]
pub struct Text<T> {
    pub origin: Coord<T>,
    pub value: String,
}

impl<T> Text<T> {
    pub fn convert<U>(self, converter: &impl Convert<T, U>) -> Text<U> {
        Text {
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
