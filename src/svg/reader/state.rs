//! A reading state of svg deserialization.

use std::{collections::HashMap, fmt::Debug};

use mlang_rs::rt::{opcode::Variable, serde::ser::Serialize};
use xml_dom::level2::{Attribute, Element, Node, NodeType, RefNode};

use crate::{
    opcode::{
        Angle, Color, FontFamily, FuncIri, Iri, Length, NumberOptNumber, Paint, PathEvent, Point,
        PreserveAspectRatio, Transform, ViewBox,
    },
    svg::{
        parse::{FromSvg, ParseError, ParseSvg},
        reader::{ReadingError, SVG_READ_REPORT},
    },
};

use super::{writer::ReadingCodeWriter, Result};

/// type value decoder for vglang core types.
pub trait Decoder {
    /// Decode one type value from [`ReadingState`] and push result back into it.
    fn decode(state: &mut ReadingState) -> Result<()>;
}

/// The operating code of xml attribute reading.
#[derive(PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum ReadingCode {
    /// Reading a string value.
    Value(String),
    /// Reading a none value.
    None,
    /// Reading a element node.
    Elem(String),
    /// Reading a leaf node.
    Leaf(String),
    /// Reading a attribute node.
    Attr(String),
    /// Reading a data node.
    Data(String),
    /// Reading a sequence start.
    SeqStart,
    /// Reading a sequence end.
    SeqEnd,
    /// Reading a enum variant.
    Variant(String),
    /// Reading a field.
    Field { name: Option<String>, index: usize },
}

impl Debug for ReadingCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReadingCode::Value(v) => write!(f, "value('{}')", v),
            ReadingCode::None => write!(f, "value(none)"),
            ReadingCode::Elem(v) => write!(f, "el({})", v),
            ReadingCode::Leaf(v) => write!(f, "leaf({})", v),
            ReadingCode::Attr(v) => write!(f, "attr({})", v),
            ReadingCode::Data(v) => write!(f, "data({})", v),
            ReadingCode::SeqStart => write!(f, "seq_start"),
            ReadingCode::SeqEnd => write!(f, "seq_end"),
            ReadingCode::Variant(v) => write!(f, "variant({})", v),
            ReadingCode::Field { name, index } => {
                if let Some(name) = name {
                    write!(f, "field({})", name)
                } else {
                    write!(f, "field({})", index)
                }
            }
        }
    }
}

/// A state of reading xml element attributes.
#[derive(Default)]
pub struct ReadingState {
    /// reading xml element tag name.
    elm_name: String,
    /// pending reading code stack.
    pub(super) codes: Vec<ReadingCode>,
    /// unread attribute/value pairs.
    attrs: HashMap<String, String>,
}

impl Drop for ReadingState {
    fn drop(&mut self) {
        if self.codes.len() > 0 {
            log::warn!(
                target: SVG_READ_REPORT,
                "unread opcodes, {:?}", self.codes
            );
        }

        if self.attrs.len() > 0 {
            log::warn!(
                target: SVG_READ_REPORT,
                "unread xml attrs, {:?}", self.attrs
            );
        }
    }
}

impl ReadingState {
    /// Push codes from [`ReadingCodeWriter`]
    pub fn push_codes(&mut self, writer: ReadingCodeWriter) {
        let codes = writer.0;
        let mut codes = codes.into_iter().rev().collect::<Vec<_>>();

        self.codes.append(&mut codes);
    }

    /// Push a code and returns the stack depth before pushing this code.
    pub fn push(&mut self, code: ReadingCode) -> usize {
        let len = self.codes.len();
        self.codes.push(code);
        len
    }

    /// Pop up codes util the providing stack depth.
    pub fn pop_d(&mut self, depth: usize) -> Option<Vec<ReadingCode>> {
        if depth > self.codes.len() {
            Some(self.codes.split_off(depth))
        } else {
            None
        }
    }

    /// Returns the topmost reading code, if exists.
    pub fn top(&self) -> Option<&ReadingCode> {
        self.codes.last()
    }

    /// Pop up the topmost `ReadingCode`.
    pub fn pop(&mut self) -> Option<ReadingCode> {
        self.codes.pop()
    }

    /// Returns  the reading opcode display name.
    pub fn opcode_name(&self) -> &str {
        &self.elm_name
    }

    pub fn attrs(&self) -> impl Iterator<Item = &str> {
        self.attrs.keys().map(|v| v.as_str())
    }

    /// Load an attribute value from cache into stack by attribute name.
    pub fn load(&mut self, name: impl AsRef<str>) {
        let value = self
            .attrs
            .remove(name.as_ref())
            .map(|v| ReadingCode::Value(v))
            .unwrap_or(ReadingCode::None);
        log::trace!(
            target: SVG_READ_REPORT,
            "{:?} load attr `{}` = {:?}",
            self.codes,
            name.as_ref(),
            value
        );
        self.codes.push(value);
    }
}

impl ReadingState {
    /// Create `ReadingState` from xml [`node`](RefNode).
    pub fn from_xml_node(node: &RefNode, child_of_defs: bool) -> Self {
        let mut attrs = HashMap::new();

        if NodeType::Text == node.node_type() {
            log::trace!(
                target: SVG_READ_REPORT,
                "deserialize xml `text` node."
            );
            attrs.insert("value".to_string(), node.to_string());

            return Self {
                elm_name: "characters".to_string(),
                codes: Default::default(),
                attrs,
            };
        }

        for (key, value) in node.attributes() {
            let key = key.to_string();

            // skip id attribute when this node is not the defs's child node.
            if !child_of_defs && key == "id" {
                continue;
            }

            if let Some(value) = value.value() {
                if key != "style" {
                    attrs.insert(key, value);
                    continue;
                }

                for kv in value.split(";") {
                    let values = kv.split(':').collect::<Vec<_>>();

                    if values.len() != 2 {
                        log::warn!(
                            target: SVG_READ_REPORT,
                            "skip unrecognized style kv: {}", kv
                        );
                        continue;
                    }

                    attrs.insert(values[0].to_string(), values[1].to_string());
                }
            }
        }

        let xml_elm = node.tag_name();

        log::trace!(
            target: SVG_READ_REPORT,
            "deserialize xml node `{}`", xml_elm
        );

        Self {
            elm_name: xml_elm,
            codes: Default::default(),
            attrs,
        }
    }

    /// Push a new [`value`](ReadingCode::Value) reading code.
    pub fn push_value<V>(&mut self, value: V)
    where
        String: From<V>,
    {
        self.codes.push(ReadingCode::Value(value.into()));
    }

    /// Push a new [`variant`](ReadingCode::Value) reading code.
    pub fn push_variant<V>(&mut self, value: V)
    where
        String: From<V>,
    {
        self.codes.push(ReadingCode::Variant(value.into()));
    }

    /// Push a new [`seq_start`](ReadingCode::SeqStart) reading code.
    pub fn push_seq_start(&mut self) {
        self.codes.push(ReadingCode::SeqStart);
    }
    /// Push a new [`seq_end`](ReadingCode::SeqEnd) reading code.
    pub fn push_seq_end(&mut self) {
        self.codes.push(ReadingCode::SeqEnd);
    }

    /// Decode a type value from the reading code stack.
    pub fn decode<V>(&mut self) -> Result<()>
    where
        V: Decoder,
    {
        V::decode(self)
    }

    /// Pop a value to read by deserializer.
    ///
    /// Cause a panic, if the topmost code is not a [`value`](ReadingCode::Value)
    pub fn pop_value(&mut self) -> Result<String> {
        match self.codes.pop() {
            Some(ReadingCode::Value(value)) => Ok(value),
            Some(ReadingCode::None) => {
                log::error!(
                    target: SVG_READ_REPORT,
                    path:serde = self.codes;
                    "{:?}, value is none.",
                    self.codes,
                );
                Err(ReadingError::None)
            }
            code => {
                log::error!(
                    target: SVG_READ_REPORT,
                    path:serde = self.codes;
                    "{:?} pop_value panic, the topmost code is {:?}",
                    self.codes,
                    code,
                );

                panic!(
                    "({}) pop_value panic, the topmost code is {:?}",
                    SVG_READ_REPORT, code
                );
            }
        }
    }

    /// Pop a `SeqStart` code.
    ///
    /// Cause a panic, if the topmost code is not a [`SeqStart`](ReadingCode::SeqStart)
    pub fn pop_seq_start(&mut self) {
        match self.codes.pop() {
            Some(ReadingCode::SeqStart) => {}
            code => {
                log::error!(
                    target: SVG_READ_REPORT,
                    path:serde = self.codes;
                    "{:?} pop_seq_start panic, the topmost code is {:?}",
                    self.codes,code
                );

                panic!(
                    "{}: pop_seq_start panic, the topmost code is {:?}",
                    SVG_READ_REPORT, code
                );
            }
        }
    }

    /// Pop the top `None` code if it exists.
    pub fn pop_none(&mut self) -> bool {
        if let Some(ReadingCode::None) = self.codes.last() {
            self.codes.pop();
            true
        } else {
            false
        }
    }

    /// Parse the topmost value as `V`.
    pub fn parse<V>(&mut self) -> Result<V>
    where
        V: FromSvg<Err = ParseError>,
    {
        match self.codes.pop() {
            Some(ReadingCode::Value(value)) => {
                let value = value.parse_svg::<V>()?;
                Ok(value)
            }
            Some(ReadingCode::None) => return Err(ReadingError::None),
            code => {
                panic!("{:?}, parse panic", code);
            }
        }
    }
}

pub(super) struct ViewBoxDecoder;

impl Decoder for ViewBoxDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        state.load("viewBox");

        let mut viewbox = state.parse::<ViewBox>()?;

        state.load("preserveAspectRatio");

        viewbox.aspect = match state.parse::<PreserveAspectRatio>() {
            Ok(aspect) => Some(Variable::Constant(aspect)),
            Err(ReadingError::None) => None,
            Err(err) => return Err(err),
        };

        let mut writer = ReadingCodeWriter::default();

        viewbox.serialize(&mut writer)?;

        state.push_codes(writer);

        Ok(())
    }
}

pub(super) struct StrokeLineJoinDecoder;

impl Decoder for StrokeLineJoinDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        let ty = state.pop_value()?;

        if ty == "miter" {
            state.load("stroke-miterlimit");
        }

        state.push_variant(ty);

        Ok(())
    }
}

pub(super) struct LengthDecoder;

impl Decoder for LengthDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        let length = state.parse::<Length>()?;

        let mut writer = ReadingCodeWriter::default();

        length.serialize(&mut writer)?;

        state.push_codes(writer);

        Ok(())
    }
}

pub(super) struct TransformListDecoder;

impl Decoder for TransformListDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        let transforms = state.parse::<Vec<Transform>>()?;

        let mut writer = ReadingCodeWriter::default();

        transforms.serialize(&mut writer)?;

        state.push_codes(writer);

        Ok(())
    }
}

pub(super) struct PaintDecoder;

impl Decoder for PaintDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        let value = state.parse::<Paint>()?;

        let mut writer = ReadingCodeWriter::default();

        value.serialize(&mut writer)?;

        state.push_codes(writer);

        Ok(())
    }
}

pub(super) struct VariantDecoder;

impl Decoder for VariantDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        let value = state.pop_value()?;

        state.push_variant(value);

        Ok(())
    }
}

pub(super) struct PathEventsDecoder;

impl Decoder for PathEventsDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        let value = state.parse::<Vec<PathEvent>>()?;

        let mut writer = ReadingCodeWriter::default();

        value.serialize(&mut writer)?;

        state.push_codes(writer);

        Ok(())
    }
}

pub(super) struct ColorDecoder;

impl Decoder for ColorDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        let value = state.parse::<Color>()?;

        let mut writer = ReadingCodeWriter::default();

        value.serialize(&mut writer)?;

        state.push_codes(writer);

        Ok(())
    }
}

pub(super) struct IriDecoder;

impl Decoder for IriDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        let value = state.parse::<Iri>()?;

        let mut writer = ReadingCodeWriter::default();

        value.serialize(&mut writer)?;

        state.push_codes(writer);

        Ok(())
    }
}

pub(super) struct FeColorMatrixDecoder;

impl Decoder for FeColorMatrixDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        state.load("type");

        let ty = state.pop_value()?;

        let ty = ty.trim();

        match ty {
            "matrix" => {
                let values = state.parse::<Vec<f32>>()?;
                state.push_seq_end();
                for value in values.into_iter().rev() {
                    state.push_value(value.to_string());
                }
                state.push_seq_start();
                state.push(ReadingCode::Field {
                    name: None,
                    index: 0,
                });
                state.push_variant("matrix");
            }
            "saturate" => {
                let value = state.parse::<f32>()?;

                state.push_value(value.to_string());
                state.push(ReadingCode::Field {
                    name: None,
                    index: 0,
                });
                state.push_variant("saturate");
            }
            "hueRotate" => {
                let value = state.parse::<f32>()?;

                state.push_value(value.to_string());

                state.push(ReadingCode::Field {
                    name: None,
                    index: 0,
                });

                state.push_variant("hueRotate");
            }
            "luminanceToAlpha" => {
                assert_eq!(state.pop(), Some(ReadingCode::None));
                state.push_variant("luminanceToAlpha");
            }
            variant => {
                log::error!(target: SVG_READ_REPORT, "unknown feColorMatrix variant `{}`",variant);
            }
        }

        Ok(())
    }
}

pub(super) struct FontFamilyListDecoder;

impl Decoder for FontFamilyListDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        let value = state.parse::<Vec<FontFamily>>()?;

        let mut writer = ReadingCodeWriter::default();

        value.serialize(&mut writer)?;

        state.push_codes(writer);

        Ok(())
    }
}

pub(super) struct AngleListDecoder;

impl Decoder for AngleListDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        let value = state.parse::<Vec<Angle>>()?;

        let mut writer = ReadingCodeWriter::default();

        value.serialize(&mut writer)?;

        state.push_codes(writer);

        Ok(())
    }
}

pub(super) struct LengthListDecoder;

impl Decoder for LengthListDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        let value = state.parse::<Vec<Length>>()?;

        let mut writer = ReadingCodeWriter::default();

        value.serialize(&mut writer)?;

        state.push_codes(writer);

        Ok(())
    }
}

pub(super) struct AngleDecoder;

impl Decoder for AngleDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        let value = state.parse::<Angle>()?;

        let mut writer = ReadingCodeWriter::default();

        value.serialize(&mut writer)?;

        state.push_codes(writer);

        Ok(())
    }
}

pub(super) struct FeFuncDecoder;

impl Decoder for FeFuncDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        state.load("type");

        let ty = state.pop_value()?;

        match ty.as_str() {
            "identity" => {}
            "table" => {
                state.load("tableValues");
                let values = state.parse::<Vec<f32>>()?;

                state.push_seq_end();

                for number in values.into_iter().rev() {
                    state.push_value(number.to_string());
                }

                state.push_seq_start();
                state.push(ReadingCode::Field {
                    name: None,
                    index: 0,
                });
            }
            "discrete" => {
                state.load("tableValues");
                let values = state.parse::<Vec<f32>>()?;

                state.push_seq_end();

                for number in values.into_iter().rev() {
                    state.push_value(number.to_string());
                }

                state.push_seq_start();

                state.push(ReadingCode::Field {
                    name: None,
                    index: 0,
                });
            }
            "linear" => {
                state.load("slope");
                let slope = state.parse::<f32>()?;

                state.load("intercept");
                let intercept = state.parse::<f32>()?;

                state.push_value(intercept.to_string());
                state.push(ReadingCode::Field {
                    name: Some("intercept".to_string()),
                    index: 1,
                });
                state.push_value(slope.to_string());
                state.push(ReadingCode::Field {
                    name: Some("slope".to_string()),
                    index: 0,
                });
            }
            "gamma" => {
                state.load("amplitude");
                let amplitude = state.parse::<f32>()?;
                state.load("exponent");
                let exponent = state.parse::<f32>()?;

                state.load("offset");
                let offset = state.parse::<f32>()?;

                state.push_value(offset.to_string());
                state.push(ReadingCode::Field {
                    name: Some("offset".to_string()),
                    index: 2,
                });
                state.push_value(exponent.to_string());
                state.push(ReadingCode::Field {
                    name: Some("exponent".to_string()),
                    index: 1,
                });
                state.push_value(amplitude.to_string());
                state.push(ReadingCode::Field {
                    name: Some("amplitude".to_string()),
                    index: 0,
                });
            }
            name => {
                log::error!(target: SVG_READ_REPORT, "unknown feFunc type {}",name);
            }
        }

        state.push_variant(ty);
        Ok(())
    }
}

pub(super) struct PointDecoder;

impl Decoder for PointDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        let value = state.parse::<Point>()?;

        let mut writer = ReadingCodeWriter::default();

        value.serialize(&mut writer)?;

        state.push_codes(writer);

        Ok(())
    }
}

pub(super) struct PointListDecoder;

impl Decoder for PointListDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        let value = state.parse::<Vec<Point>>()?;

        let mut writer = ReadingCodeWriter::default();

        value.serialize(&mut writer)?;

        state.push_codes(writer);

        Ok(())
    }
}

pub(super) struct FuncIriDecoder;

impl Decoder for FuncIriDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        let value = state.parse::<FuncIri>()?;

        let mut writer = ReadingCodeWriter::default();

        value.serialize(&mut writer)?;

        state.push_codes(writer);

        Ok(())
    }
}

pub(super) struct NumberOptNumberDecoder;

impl Decoder for NumberOptNumberDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        let value = state.parse::<NumberOptNumber>()?;

        let mut writer = ReadingCodeWriter::default();

        value.serialize(&mut writer)?;

        state.push_codes(writer);

        Ok(())
    }
}

pub(super) struct CharactersDecoder;

impl Decoder for CharactersDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        state.load("value");
        state.push(ReadingCode::Field {
            name: None,
            index: 0,
        });
        Ok(())
    }
}

pub(super) struct FeInDecoder;

impl Decoder for FeInDecoder {
    fn decode(state: &mut ReadingState) -> Result<()> {
        let value = state.pop_value()?;

        let value = value.trim();

        match value {
            "SourceGraphic" | "SourceAlpha" | "BackgroundImage" | "BackgroundAlpha"
            | "FillPaint" | "StrokePaint" => {
                state.push_variant(value);
            }
            _ => {
                state.push_value(value);
                state.push(ReadingCode::Field {
                    name: None,
                    index: 0,
                });
                state.push_variant("result");
            }
        }

        Ok(())
    }
}
