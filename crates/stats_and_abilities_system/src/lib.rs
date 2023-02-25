pub mod event_queue;
pub mod health;
pub mod ressource;
pub mod stats;

pub type StatValueType = i32;
pub type StatModifierType = f32;
pub type StatIdentifier = String;

pub mod prelude {
    pub use crate::health::*;
    pub use crate::stats::*;
}
