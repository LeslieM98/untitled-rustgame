#![allow(dead_code)]
pub mod health;
pub mod stats;

pub type StatValueType = i32;
pub type StatUValueType = u32;
pub type StatFloatType = f32;
pub type StatIdentifier = String;

pub mod prelude {
    pub use crate::health::*;
    pub use crate::stats::*;
}
