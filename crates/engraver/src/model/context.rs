use std::{collections::HashMap, mem};

use smufl::StaffSpaces;

use super::{Alteration, KeySignature, Pitch, Step};

#[derive(Debug)]
pub struct Context {
    pub middle_c_position: StaffSpaces,
    key_signature: Option<KeySignature>,
    previous_measure_alterations: HashMap<(Step, i8), Alteration>,
    current_measure_alterations: HashMap<(Step, i8), Alteration>,
}

#[derive(Debug)]
pub enum AccidentalState {
    NeedsAccidental,
    NoAccidental,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            middle_c_position: StaffSpaces(-1.0),
            key_signature: Default::default(),
            previous_measure_alterations: Default::default(),
            current_measure_alterations: Default::default(),
        }
    }
}

impl Context {
    pub fn start_key_signature(&mut self, key_signature: KeySignature) {
        self.key_signature = Some(key_signature)
    }

    pub fn start_measure(&mut self) {
        self.previous_measure_alterations = mem::take(&mut self.current_measure_alterations);
    }

    pub fn add_pitch(&mut self, pitch: Pitch) -> AccidentalState {
        let key = (pitch.step, pitch.octave);

        let previous_measure_alteration = self.previous_measure_alterations.get(&key);
        let current_measure_alteration = self.current_measure_alterations.get(&key);
        let key_signature_alteration = self
            .key_signature
            .map(|key_signature| key_signature.alterations())
            .unwrap_or_default()
            .get(&pitch.step)
            .copied()
            .unwrap_or(Alteration::Natural);

        let accidental_state = match (previous_measure_alteration, current_measure_alteration) {
            (_, Some(current_measure_alteration))
                if pitch.alteration != *current_measure_alteration =>
            {
                AccidentalState::NeedsAccidental
            }
            (_, Some(_)) => AccidentalState::NoAccidental,
            (Some(previous_measure_alteration), None)
                if pitch.alteration != *previous_measure_alteration =>
            {
                AccidentalState::NeedsAccidental
            }
            _ if pitch.alteration != key_signature_alteration => AccidentalState::NeedsAccidental,
            _ => AccidentalState::NoAccidental,
        };

        self.previous_measure_alterations.remove(&key);
        self.current_measure_alterations
            .insert(key, pitch.alteration);

        accidental_state
    }
}
