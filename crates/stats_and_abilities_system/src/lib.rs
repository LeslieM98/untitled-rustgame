pub mod health;
pub mod stats;

pub type StatValueType = i32;
pub type StatModifierType = f32;
pub type StatIdentifier = String;

pub mod prelude {

    pub use crate::StatIdentifier;
    pub use crate::StatModifierType;
    pub use crate::StatIdentifier;

    pub use crate::stats::*;
    pub use crate::health::*;
}
