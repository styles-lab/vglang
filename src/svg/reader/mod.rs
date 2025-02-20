//! A deserializer of svg format.

mod error;
pub use error::*;

pub mod state;
pub mod writer;

mod reader;
pub use reader::*;
