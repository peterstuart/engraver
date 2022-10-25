pub mod input;
pub mod ir;

mod engraving_defaults_extensions;
mod metadata_extensions;
mod stem_direction;

use smufl::{Metadata, StaffSpaces};

use self::{ir::Element, stem_direction::StemDirection};

pub trait Render {
    fn render(&self, x: StaffSpaces, metadata: &Metadata) -> Output;
}

#[derive(Clone, Debug)]
pub struct Output {
    pub elements: Vec<Element<StaffSpaces>>,
    pub width: StaffSpaces,
}

pub struct Renderer<'m> {
    elements: Vec<Element<StaffSpaces>>,
    position: StaffSpaces,
    metadata: &'m Metadata,
}

impl<'m> Renderer<'m> {
    pub fn new(metadata: &'m Metadata) -> Self {
        Self {
            elements: vec![],
            position: StaffSpaces::zero(),
            metadata,
        }
    }

    pub fn add_elements(&mut self, mut elements: Vec<Element<StaffSpaces>>) -> &mut Self {
        self.elements.append(&mut elements);

        self
    }

    pub fn render<T: Render>(&mut self, render: &T) -> &mut Self {
        let mut output = render.render(self.position, self.metadata);

        self.elements.append(&mut output.elements);
        self.position += output.width;

        self
    }

    pub fn advance(&mut self, width: StaffSpaces) -> &mut Self {
        self.position += width;

        self
    }

    pub fn position(&self) -> StaffSpaces {
        self.position
    }

    pub fn to_elements(self) -> Vec<Element<StaffSpaces>> {
        self.elements
    }
}
