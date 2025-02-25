/// svg attribute value parsing error.
#[derive(Debug, thiserror::Error, PartialEq, Clone)]
pub enum ParseError {
    #[error("failed parsing svg `{0}` value: `{1}`")]
    Failed(ParseKind, String),
    #[error("failed parsing svg `{0}` value, overflow: {0}")]
    Overflow(ParseKind, String),
    #[error("failed parsing svg `{0}` value, unparsing: `{0}`")]
    Unparsed(ParseKind, String),
}

impl ParseError {
    pub(crate) fn failed(kind: ParseKind, source: impl AsRef<str>) -> Self {
        Self::Failed(kind, source.as_ref().to_string())
    }

    #[allow(unused)]
    pub(crate) fn overflow(kind: ParseKind, source: impl AsRef<str>) -> Self {
        Self::Overflow(kind, source.as_ref().to_string())
    }

    pub(crate) fn unparsed(kind: ParseKind, source: impl AsRef<str>) -> Self {
        Self::Unparsed(kind, source.as_ref().to_string())
    }
}
/// svg attribute value parsing error.
#[derive(Debug, thiserror::Error, PartialEq, Clone)]
pub enum ParseKind {
    #[error("integer")]
    Integer,
    #[error("number")]
    Number,
    #[error("numbers")]
    Numbers,
    #[error("number-optional-number")]
    NumberOptNumber,
    #[error("point")]
    Point,
    #[error("points")]
    Points,
    #[error("color")]
    Color,
    #[error("iri")]
    Iri,
    #[error("funciri")]
    FuncIri,
    #[error("paint")]
    Paint,
    #[error("font-family")]
    FontFamily,
    #[error("angle")]
    Angle,
    #[error("length")]
    Length,
    #[error("angle-list")]
    Angles,
    #[error("length-list")]
    Lengths,
    #[error("viewBox")]
    ViewBox,
    #[error("preserveAspectRatio")]
    PreserveAspectRatio,
    #[error("transform-list")]
    TransformList,
    #[error("background")]
    Background,
    #[error("path-event")]
    PathEvent,
    #[error("path-event-list")]
    PathEvents,
}

/// Parsing result type, returns by this module functions.
pub type Result<T> = std::result::Result<T, ParseError>;

pub(super) const SVG_PARSE_ERROR: &str = "SVG_PARSE_ERROR";
