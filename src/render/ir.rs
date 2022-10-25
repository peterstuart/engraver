use std::ops::Add;

#[derive(Clone, Copy, Debug)]
pub struct Coord<T> {
    pub x: T,
    pub y: T,
}

impl<T> Coord<T> {
    pub fn convert<U, F: Fn(T) -> U>(self, f: F) -> Coord<U> {
        Coord {
            x: f(self.x),
            y: f(self.y),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}

impl<T> Size<T> {
    pub fn convert<U, F: Fn(T) -> U>(self, f: F) -> Size<U> {
        Size {
            width: f(self.width),
            height: f(self.height),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Element<T> {
    Line(Line<T>),
    Rect(Rect<T>),
    Symbol(Symbol<T>),
    Text(Text<T>),
}

impl<T> Element<T> {
    pub fn convert<U, F: Fn(T) -> U>(self, f: &F) -> Element<U> {
        match self {
            Self::Line(line) => Element::Line(line.convert(f)),
            Self::Rect(rect) => Element::Rect(rect.convert(f)),
            Self::Symbol(symbol) => Element::Symbol(symbol.convert(f)),
            Self::Text(text) => Element::Text(text.convert(f)),
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
    pub fn convert<U, F: Fn(T) -> U>(self, f: F) -> Line<U> {
        Line {
            from: self.from.convert(&f),
            to: self.to.convert(&f),
            thickness: f(self.thickness),
            cap: self.cap,
        }
    }

    pub fn max_x(&self) -> T
    where
        T: Copy + PartialOrd,
    {
        if self.from.x > self.to.x {
            self.from.x
        } else {
            self.to.x
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Rect<T> {
    pub origin: Coord<T>,
    pub size: Size<T>,
}

impl<T> Rect<T> {
    pub fn convert<U, F: Fn(T) -> U>(self, f: F) -> Rect<U> {
        Rect {
            origin: self.origin.convert(&f),
            size: self.size.convert(&f),
        }
    }

    pub fn max_x(&self) -> T
    where
        T: Add<Output = T> + Copy,
    {
        self.origin.x + self.size.width
    }
}

#[derive(Clone, Debug)]
pub struct Symbol<T> {
    pub origin: Coord<T>,
    pub value: char,
}

impl<T> Symbol<T> {
    pub fn convert<U, F: Fn(T) -> U>(self, f: F) -> Symbol<U> {
        Symbol {
            origin: self.origin.convert(f),
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

#[derive(Clone, Debug)]
pub struct Text<T> {
    pub origin: Coord<T>,
    pub value: String,
}

impl<T> Text<T> {
    pub fn convert<U, F: Fn(T) -> U>(self, f: F) -> Text<U> {
        Text {
            origin: self.origin.convert(f),
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
