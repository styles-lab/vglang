/// Reading error returns by this module.
#[derive(Debug, thiserror::Error)]
pub enum WritingError {
    #[error(transparent)]
    DomErr(#[from] xml_dom::level2::Error),
    #[error("svg does not support variable opcode.")]
    Variable,

    #[error("The root node of a valid svg document is `svg`.")]
    Root,
}

/// Reading result returns by functions in this module .
pub type Result<T> = std::result::Result<T, WritingError>;

#[allow(unused)]
pub(super) const SVG_WRITE_REPORT: &str = "SVG_WRITE_REPORT";
