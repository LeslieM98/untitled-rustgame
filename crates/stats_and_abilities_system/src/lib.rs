pub mod stats;
pub type StatValueType = i32;
pub type StatModifierType = f32;
pub type StatIdentifier = String;

pub mod prelude {

    pub use crate::stats::*;
}
