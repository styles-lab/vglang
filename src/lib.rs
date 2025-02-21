//! **vglang** is a simple and fast *`Vector Graphics programming Language`*

#![cfg_attr(docsrs, feature(doc_cfg))]

mod ml;

/// Svg-compatible high-level opcodes.
pub mod opcode {
    pub use super::ml::opcode::*;
}

#[cfg(feature = "svg")]
#[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
pub mod svg;
