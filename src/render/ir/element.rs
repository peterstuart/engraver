use std::ops::{Add, Sub};

use super::{Convert, Line, Rect, Symbol, Text};

#[derive(Clone, Debug)]
pub enum Element<T> {
    Line(Line<T>),
    Rect(Rect<T>),
    Symbol(Symbol<T>),
    Text(Text<T>),
}

impl<T> Element<T> {
    pub fn convert<U>(self, converter: &impl Convert<T, U>) -> Element<U>
    where
        T: Add<Output = T> + Copy,
        U: Copy + PartialOrd + Sub<Output = U>,
    {
        match self {
            Self::Line(line) => Element::Line(line.convert(converter)),
            Self::Rect(rect) => Element::Rect(rect.convert(converter)),
            Self::Symbol(symbol) => Element::Symbol(symbol.convert(converter)),
            Self::Text(text) => Element::Text(text.convert(converter)),
        }
    }

    pub fn max_x(&self) -> T
    where
        T: Add<Output = T> + Copy + PartialOrd,
    {
        match self {
            Element::Line(line) => line.max_x(),
            Element::Rect(rect) => rect.max_x(),
            Element::Symbol(symbol) => symbol.max_x(),
            Element::Text(text) => text.max_x(),
        }
    }
}
