use super::{Convert, Element};

#[derive(Clone, Debug)]
pub struct Group<T> {
    pub elements: Vec<Element<T>>,
    pub id: Option<String>,
}

impl<T> Group<T> {
    pub fn convert<U>(self, converter: &impl Convert<T, U>) -> Group<U> {
        Group {
            id: self.id,
            elements: self
                .elements
                .into_iter()
                .map(|e| e.convert(converter))
                .collect(),
        }
    }

    pub fn max_x(&self) -> T
    where
        T: Copy + PartialOrd,
    {
        self.elements
            .iter()
            .max_by(|a, b| a.max_x().partial_cmp(&b.max_x()).unwrap())
            .unwrap()
            .max_x()
    }
}
