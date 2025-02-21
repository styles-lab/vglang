//! This module provides serialisation/deserialisation support for opcodes in binary format.
//!
//! [`opcode`]: super::opcode

mod keyword;

mod reader;
pub use reader::*;
mod writer;
pub use writer::*;
