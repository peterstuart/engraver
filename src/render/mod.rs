pub mod context;
pub mod input;
pub mod ir;
pub mod stem;

mod engraving_defaults_extensions;
mod glyph_data_extensions;
mod math;
mod metadata_extensions;

use smufl::{Metadata, StaffSpaces};

use self::{context::Context, ir::Element};
use crate::Result;

pub trait Render {
    fn render(&self, x: StaffSpaces, context: &mut Context, metadata: &Metadata) -> Result<Output>;
}

#[derive(Clone, Debug)]
pub struct Output {
    pub elements: Vec<Element<StaffSpaces>>,
    pub width: StaffSpaces,
}

pub struct Renderer<'m> {
    elements: Vec<Element<StaffSpaces>>,
    position: StaffSpaces,
    context: Context,
    metadata: &'m Metadata,
}

impl<'m> Renderer<'m> {
    pub fn new(metadata: &'m Metadata) -> Self {
        Self {
            elements: vec![],
            position: StaffSpaces::zero(),
            context: Context::default(),
            metadata,
        }
    }

    pub fn add_elements(&mut self, mut elements: Vec<Element<StaffSpaces>>) -> &mut Self {
        self.elements.append(&mut elements);

        self
    }

    pub fn context(&mut self) -> &mut Context {
        &mut self.context
    }

    pub fn render<T: Render>(&mut self, render: &T) -> Result<&mut Self> {
        let mut output = render.render(self.position, &mut self.context, self.metadata)?;

        self.elements.append(&mut output.elements);
        self.position += output.width;

        Ok(self)
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
