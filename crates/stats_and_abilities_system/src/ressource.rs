use std::collections::HashMap;

use crate::{StatIdentifier, StatValueType};

pub struct Ressource {
    identifier: StatIdentifier,
    current: StatValueType,
    maximum: StatValueType,
}

pub struct RessourceBundle {
    ressources: HashMap<StatIdentifier, StatValueType>,
}

impl Ressource {
    pub fn new(identifier: StatIdentifier, maximum: StatValueType) -> Self {
        Self {
            identifier,
            maximum,
            current: maximum,
        }
    }

    pub fn get_current(&self) -> &StatValueType {
        &self.current
    }

    pub fn get_maximum(&self) -> &StatValueType {
        &self.maximum
    }

    pub fn get_identifier(&self) -> &StatIdentifier {
        &self.identifier
    }

    pub fn get_percantage(&self) -> f32 {
        self.current as f32 / self.maximum as f32
    }
}
