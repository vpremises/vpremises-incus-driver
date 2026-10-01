//! Converts a resolved vPremises environment into provider-owned `OpenTofu` JSON.

mod model;
mod render;
mod validate;

pub use model::ResolvedEnvironment;
pub use render::render;
pub use validate::validate;

pub type Result<T> = std::result::Result<T, String>;
