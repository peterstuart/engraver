use super::{Clef, Context, KeySignature, Measure, TimeSignature};
use crate::render;

#[derive(Clone, Debug, PartialEq)]
pub struct Staff {
    pub clef: Clef,
    pub key_signature: Option<KeySignature>,
    pub time_signature: Option<TimeSignature>,
    pub measures: Vec<Measure>,
}

impl From<Staff> for render::input::Staff {
    fn from(value: Staff) -> Self {
        let mut context = Context::default();

        Self {
            clef: Some(value.clef.into_input(&mut context)),
            key_signature: value
                .key_signature
                .map(|key_signature| key_signature.into_input(&mut context)),
            time_signature: value.time_signature,
            measures: value
                .measures
                .into_iter()
                .map(|measure| measure.into_input(&mut context))
                .collect(),
        }
    }
}
