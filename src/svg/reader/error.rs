use crate::svg::parse::ParseError;

/// Reading error returns by this module.
#[derive(Debug, thiserror::Error)]
pub enum ReadingError {
    #[error(transparent)]
    Parse(#[from] ParseError),

    #[error("target is none")]
    None,

    #[error(transparent)]
    LoadXml(#[from] xml_dom::parser::Error),

    #[error("failed loading svg element from xml document.")]
    LoadSvgElement,

    #[error(transparent)]
    DerError(#[from] mlang_rs::rt::serde::de::Error),
}

/// Reading result returns by functions in this module .
pub type Result<T> = std::result::Result<T, ReadingError>;

pub(super) const SVG_READ_REPORT: &str = "SVG_READ_REPORT";
