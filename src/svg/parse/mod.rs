//! this module provides [`FromSvg`] implementations for vglang core opcode types.

mod errors;
pub use errors::*;

mod sep;

mod number;

mod angle;
mod background;
mod font;
mod iri;
mod length;
mod paint;
mod path;
mod point;
mod transform;
mod variant;
mod viewbox;

/// Parse vglang data value from svg str.
pub trait FromSvg: Sized {
    type Err;
    fn from_svg(s: &str) -> std::result::Result<Self, Self::Err>;
}

/// Extension trait for string to parse vglang data type from it.
pub trait ParseSvg {
    /// parse vglang data type from svg string.
    fn parse_svg<V: FromSvg>(&self) -> std::result::Result<V, V::Err>;
}

impl<T> ParseSvg for T
where
    T: AsRef<str>,
{
    fn parse_svg<V: FromSvg>(&self) -> std::result::Result<V, V::Err> {
        V::from_svg(self.as_ref())
    }
}
