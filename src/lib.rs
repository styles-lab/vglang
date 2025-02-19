//! **vglang** is a simple and fast *`Vector Graphics programming Language`*

#![cfg_attr(docsrs, feature(doc_cfg))]

mod ml;
pub use ml::opcode;

#[cfg(feature = "svg")]
#[cfg_attr(docsrs, doc(cfg(feature = "svg")))]
pub mod svg;
