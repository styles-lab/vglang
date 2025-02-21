//! This module provides binary format serialisation/deserialisation support for [`opcode`].
//!
//! [`opcode`]: super::opcode

mod keyword;

mod reader;
pub use reader::*;
mod writer;
pub use writer::*;
