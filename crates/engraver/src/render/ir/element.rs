use super::{Convert, Group, Line, Polygon, Symbol, Text};

#[derive(Clone, Debug)]
pub enum Element<T> {
    Line(Line<T>),
    Polygon(Polygon<T>),
    Symbol(Symbol<T>),
    Text(Text<T>),
    Group(Group<T>),
}

impl<T> Element<T> {
    pub fn convert<U>(self, converter: &impl Convert<T, U>) -> Element<U> {
        match self {
            Self::Line(line) => Element::Line(line.convert(converter)),
            Self::Polygon(polyogn) => Element::Polygon(polyogn.convert(converter)),
            Self::Symbol(symbol) => Element::Symbol(symbol.convert(converter)),
            Self::Text(text) => Element::Text(text.convert(converter)),
            Self::Group(group) => Element::Group(group.convert(converter)),
        }
    }

    pub fn max_x(&self) -> T
    where
        T: Copy + PartialOrd,
    {
        match self {
            Element::Line(line) => line.max_x(),
            Element::Polygon(polygon) => polygon.max_x(),
            Element::Symbol(symbol) => symbol.max_x(),
            Element::Text(text) => text.max_x(),
            Element::Group(group) => group.max_x(),
        }
    }
}
