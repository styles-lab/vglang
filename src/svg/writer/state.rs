use std::{collections::HashMap, fmt::Debug};

use xml_dom::level2::{Document, Element, Node, RefNode};

use crate::svg::writer::SVG_WRITE_REPORT;

use super::Result;

/// Operand used by [`WriteCode::Encoder`]
pub(super) trait Encoder: Debug {
    /// encode new data.
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState);
}

/// Opcode used by [`WriteState`]
#[derive(Debug)]
pub(super) enum WriteCode {
    None,
    /// encoded data value (string, byte, ..., data or enum).
    Value(
        /// value data in string format.
        String,
    ),
    /// Encoder to encode values.
    Encoder(Box<dyn Encoder + 'static>),
    InserAttr(
        /// Attribute name.
        String,
    ),
}

/// Serialization state machine.
#[derive(Default)]
pub(super) struct WriteState {
    /// Stack of opcodes not yet executed.
    opcodes: Vec<WriteCode>,
    /// Serialized attributes/value pairs.
    attrs: HashMap<String, String>,
}

impl WriteState {
    pub fn clear(&mut self) {
        self.opcodes.clear();
    }

    pub fn push_none(&mut self) {
        self.opcodes.push(WriteCode::None);
    }

    /// Push an [`InserAttr`] opcode.
    pub fn push_insert_attr<S>(&mut self, value: S)
    where
        String: From<S>,
    {
        self.opcodes.push(WriteCode::InserAttr(value.into()));
    }

    /// Push a new value into serialization stack.
    pub fn push_value<S>(&mut self, value: S)
    where
        String: From<S>,
    {
        self.opcodes.push(WriteCode::Value(value.into()));
    }

    /// Push a new value into serialization stack.
    pub fn push_encoder<E>(&mut self, value: E)
    where
        E: Encoder + 'static,
    {
        self.opcodes.push(WriteCode::Encoder(Box::new(value)));
    }

    /// Pop the topmost encoder in the stack and its arguments,
    /// call the encoding and push the result back into the opcodes stack.
    pub fn encode(&mut self) -> Result<()> {
        if self
            .opcodes
            .iter()
            .rev()
            .find(|op| {
                if let WriteCode::Encoder(_) = op {
                    true
                } else {
                    false
                }
            })
            .is_none()
        {
            return Ok(());
        }

        let mut encoder = None;
        let mut params = vec![];

        while let Some(code) = self.opcodes.pop() {
            match code {
                WriteCode::None => params.push(None),
                WriteCode::Value(value) | WriteCode::InserAttr(value) => params.push(Some(value)),
                WriteCode::Encoder(value) => {
                    encoder = Some(value);
                    break;
                }
            }
        }

        let params = params.into_iter().rev().collect::<Vec<_>>();

        let encoder = encoder.unwrap();

        encoder.encode(params, self);

        Ok(())
    }

    /// Find the topmost [`InserAttr`](WriteCode::InserAttr) code and invoke it.
    pub fn insert_attr(&mut self) {
        assert!(
            self.opcodes.len() > 1,
            "WriteState(insert_attr): opcodes length is less than 2."
        );

        let value = match self.opcodes.pop().unwrap() {
            WriteCode::Value(value) => value,
            _ => {
                panic!("WriteState(insert_attr): -1 value is not `WriteCode::Value`")
            }
        };

        let name = match self.opcodes.pop().unwrap() {
            WriteCode::InserAttr(name) => name,
            _ => {
                panic!("WriteState(insert_attr): -2 value is not `WriteCode::InserAttr`")
            }
        };

        self.attrs.insert(name, value);
    }

    pub fn write(mut self, node: &mut RefNode) -> Result<()> {
        let name = node.tag_name();
        let len = self.opcodes.len();

        if len % 2 != 0 {
            log::error!(
                target: SVG_WRITE_REPORT,
                "write attrs({}): the count of opcodes is odd: {:?}",
                name,
                self.opcodes
            );

            panic!("write attrs({}): the count of opcodes is odd", name);
        }

        for i in 0..len / 2 {
            let name = match &self.opcodes[i * 2] {
                WriteCode::InserAttr(name) => name,
                _ => {
                    panic!(
                        "WriteState(into_xml_attrs): opcodes[{}] is not `InserAttr`",
                        i * 2
                    );
                }
            };

            let value = match &self.opcodes[i * 2 + 1] {
                WriteCode::Value(v) => v,
                WriteCode::None => {
                    // skip this attribute.
                    continue;
                }
                _ => {
                    panic!(
                        "WriteState(into_xml_attrs): opcodes[{}] is not `Value`, {:?}",
                        i * 2 + 1,
                        self.opcodes[i * 2 + 1]
                    );
                }
            };

            self.attrs.insert(name.to_string(), value.to_string());
        }

        for (name, value) in self.attrs {
            node.set_attribute(&name, &value)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub(super) struct CharactersWriter {
    pub(super) document: RefNode,
    pub(super) node: RefNode,
}

impl Encoder for CharactersWriter {
    fn encode(&self, params: Vec<Option<String>>, _: &mut WriteState) {
        assert_eq!(params.len(), 2, "encode(Characters): params != 1");

        self.node
            .clone()
            .append_child(
                self.document.create_text_node(
                    params[1]
                        .as_ref()
                        .expect("encode(Characters): value is none."),
                ),
            )
            .expect("insert characters");
    }
}

#[derive(Debug)]
pub(super) struct AngleWriter(
    /// unit of angle.
    pub(super) String,
);

impl Encoder for AngleWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        assert_eq!(params.len(), 2, "encode: angle params != 1");

        if self.0 == "deg" {
            state.push_value(params[1].as_ref().expect("angle value is none."));
        } else {
            state.push_value(format!(
                "{}{}",
                params[1].as_ref().expect("angle value is none."),
                self.0
            ));
        }
    }
}
#[derive(Debug)]
pub(super) struct ColorWriter(
    /// unit of angle.
    pub(super) String,
);

impl Encoder for ColorWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        match self.0.as_str() {
            "keyword" => {
                assert_eq!(params.len(), 2, "encode(Color): keyword params != 1");
                state.push_value(
                    params[1]
                        .as_ref()
                        .expect("encode(Color): keyword param is none."),
                );
            }
            "rgb" => {
                assert_eq!(params.len(), 6, "encode(Color): color params != 3");
                state.push_value(format!(
                    "rgb({},{},{})",
                    params[1]
                        .as_ref()
                        .expect("encode(Color): color red is none."),
                    params[3]
                        .as_ref()
                        .expect("encode(Color): color green is none."),
                    params[5]
                        .as_ref()
                        .expect("encode(Color): color blue is none.")
                ));
            }
            varaint => {
                log::error!(target: SVG_WRITE_REPORT,"unknown color variant `{}`",varaint);
            }
        }
    }
}

#[derive(Debug)]
pub(super) struct StrokeLineJoinWriter(
    /// linejoin type.
    pub(super) String,
);

impl Encoder for StrokeLineJoinWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        assert_eq!(params.len(), 2, "encode: angle params != 1");

        if self.0 == "miter" {
            if let Some(value) = &params[1] {
                state.push_insert_attr("stroke-miterlimit");
                state.push_value(value);
                state.insert_attr();
            }
        }

        state.push_value(&self.0);
    }
}

#[derive(Debug)]
pub(super) struct LengthWriter(
    /// unit of length.
    pub(super) String,
);

impl Encoder for LengthWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        assert_eq!(params.len(), 2, "encode: length params != 1");

        if self.0 == "px" {
            state.push_value(params[1].as_ref().expect("length value is none."));
        } else if self.0 == "percent" {
            state.push_value(format!(
                "{}%",
                params[1].as_ref().expect("length value is none."),
            ));
        } else {
            state.push_value(format!(
                "{}{}",
                params[1].as_ref().expect("length value is none."),
                self.0
            ));
        }
    }
}

#[derive(Debug)]
pub(super) struct RgbWriter;

impl Encoder for RgbWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        assert_eq!(params.len(), 6, "encode: rgb params != 3");

        state.push_value(format!(
            "rgb({},{},{})",
            params[1].as_ref().expect("rgb: red component is none."),
            params[3].as_ref().expect("rgb: green component is none."),
            params[5].as_ref().expect("rgb: blue component is none."),
        ));
    }
}

#[derive(Debug)]
pub(super) struct IriWriter(
    /// is local iri.
    pub(super) String,
);

impl Encoder for IriWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        assert_eq!(params.len(), 2, "encode: iri params != 1");
        if self.0 == "local" {
            state.push_value(format!(
                "#{}",
                params[1].as_ref().expect("Iri value is none.")
            ));
        } else {
            state.push_value(params[1].as_ref().expect("Iri value is none."));
        }
    }
}

#[derive(Debug)]
pub(super) struct FuncIriWriter;

impl Encoder for FuncIriWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        assert_eq!(params.len(), 2, "encode: funciri params != 1");
        state.push_value(format!(
            "url(#{})",
            params[1].as_ref().expect("encode: funciri value is none.")
        ));
    }
}

#[derive(Debug)]
pub(super) struct PointWriter;

impl Encoder for PointWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        assert_eq!(params.len(), 4, "encode: point params != 2");

        state.push_value(format!(
            "{} {}",
            params[1].as_ref().expect("encode: point x is none."),
            params[3].as_ref().expect("encode: point y is none.")
        ));
    }
}

#[derive(Debug)]
pub(super) struct PercentWriter;

impl Encoder for PercentWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        assert_eq!(params.len(), 2, "encode: percent params != 1");

        state.push_value(format!(
            "{}%",
            params[1].as_ref().expect("encode: percent value is none.")
        ));
    }
}

#[derive(Debug)]
pub(super) struct EnumWriter {
    pub(super) ty: String,
    /// variant name.
    pub(super) variant: String,
    /// count of fields of the variant.
    pub(super) fields: usize,
}

impl Encoder for EnumWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        // variant without params.
        match self.fields {
            0 => {
                assert_eq!(
                    params.len(),
                    0,
                    "encode: enum({}::{}) params != 0",
                    self.ty,
                    self.variant
                );
                state.push_value(&self.variant);
            }
            1 => {
                assert_eq!(
                    params.len(),
                    2,
                    "encode: enum({}::{}) params != 1",
                    self.ty,
                    self.variant
                );

                state.push_value(params[1].as_ref().expect(&format!(
                    "encode: {}::{} value is none.",
                    self.ty, self.variant
                )));
            }
            _ => {
                panic!("Unsupport enum variant: {}::{}", self.ty, self.variant);
            }
        }
    }
}

#[derive(Debug)]
pub(super) struct NumberOptNumberWriter;

impl Encoder for NumberOptNumberWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        assert_eq!(params.len(), 4, "encode: NumberOptNumber params != 2",);

        if let Some(value2) = &params[3] {
            state.push_value(format!(
                "{},{}",
                params[1]
                    .as_ref()
                    .expect("encode: NumberOptNumber value(0) is none."),
                value2
            ));
        } else {
            state.push_value(
                params[1]
                    .as_ref()
                    .expect("encode: NumberOptNumber value(0) is none."),
            );
        }
    }
}

#[derive(Debug)]
pub(super) struct TransformWriter(
    /// variant name.
    pub(super) String,
);

impl Encoder for TransformWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        match self.0.as_str() {
            "matrix" => {
                assert_eq!(params.len(), 2, "encode: transform(matrix) params != 1");

                state.push_value(format!(
                    "{}({})",
                    self.0,
                    params[1]
                        .as_ref()
                        .expect(&format!("encode: transform(matrix) value 0 is none.",)),
                ));
            }
            "translate" | "scale" | "rotate" => {
                assert_eq!(params.len(), 4, "encode: transform({}) params != 2", self.0);
                if let Some(y) = &params[3] {
                    state.push_value(format!(
                        "{}({},{})",
                        self.0,
                        params[1]
                            .as_ref()
                            .expect(&format!("encode: transform({}) value 0 is none.", self.0)),
                        y
                    ));
                } else {
                    state.push_value(format!(
                        "{}({})",
                        self.0,
                        params[1]
                            .as_ref()
                            .expect(&format!("encode: transform({}) value 0 is none.", self.0)),
                    ));
                }
            }
            "skewX" | "skewY" => {
                assert_eq!(params.len(), 2, "encode: transform({}) params !=1", self.0);

                state.push_value(format!(
                    "{}({})",
                    self.0,
                    params[1]
                        .as_ref()
                        .expect(&format!("encode: transform({}) value 0 is none.", self.0)),
                ));
            }
            _ => {
                panic!("Unsupport transform type: {}", self.0)
            }
        }
    }
}

#[derive(Debug)]
pub(super) struct PathEventWriter(
    /// variant name.
    pub(super) String,
);

impl Encoder for PathEventWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        match self.0.as_str() {
            "close" => {
                assert_eq!(params.len(), 0, "encode(pathEvent): close params != 0");
                state.push_value("z");
            }
            "moveTo" => {
                assert_eq!(params.len(), 4, "encode(pathEvent): moveTo params != 4");

                if params[3].as_ref().expect("") == "1" {
                    state.push_value(format!(
                        "m {}",
                        params[1]
                            .as_ref()
                            .expect("encode(pathEvent): moveTo(points) is none.")
                    ));
                } else {
                    state.push_value(format!(
                        "M {}",
                        params[1]
                            .as_ref()
                            .expect("encode(pathEvent): moveTo(points) is none.")
                    ));
                }
            }
            "lineTo" => {
                assert_eq!(params.len(), 4, "encode(pathEvent): lineTo params != 2");

                if params[3].as_ref().expect("") == "1" {
                    state.push_value(format!(
                        "l {}",
                        params[1]
                            .as_ref()
                            .expect("encode(pathEvent): lineTo param is none.")
                    ));
                } else {
                    state.push_value(format!(
                        "L {}",
                        params[1]
                            .as_ref()
                            .expect("encode(pathEvent): lineTo param is none.")
                    ));
                }
            }
            "horizontal" => {
                assert_eq!(params.len(), 4, "encode(pathEvent): horizontal params != 2");
                if params[3].as_ref().expect("") == "1" {
                    state.push_value(format!(
                        "h {}",
                        params[1]
                            .as_ref()
                            .expect("encode(pathEvent): horizontal param is none.")
                    ));
                } else {
                    state.push_value(format!(
                        "H {}",
                        params[1]
                            .as_ref()
                            .expect("encode(pathEvent): horizontal param is none.")
                    ));
                }
            }
            "vertical" => {
                assert_eq!(params.len(), 4, "encode(pathEvent): vertical params != 2");

                if params[3].as_ref().expect("") == "1" {
                    state.push_value(format!(
                        "v {}",
                        params[1]
                            .as_ref()
                            .expect("encode(pathEvent): vertical param is none.")
                    ));
                } else {
                    state.push_value(format!(
                        "V {}",
                        params[1]
                            .as_ref()
                            .expect("encode(pathEvent): vertical param is none.")
                    ));
                }
            }
            "cubicBezier" => {
                assert_eq!(
                    params.len(),
                    4,
                    "encode(pathEvent): cubicBezier params != 2"
                );

                if params[3].as_ref().expect("") == "1" {
                    state.push_value(format!(
                        "c {}",
                        params[1]
                            .as_ref()
                            .expect("encode(pathEvent): cubicBezier param is none.")
                    ));
                } else {
                    state.push_value(format!(
                        "C {}",
                        params[1]
                            .as_ref()
                            .expect("encode(pathEvent): cubicBezier param is none.")
                    ));
                }
            }
            "cubicBezierSmooth" => {
                assert_eq!(
                    params.len(),
                    4,
                    "encode(pathEvent): cubicBezierSmooth params != 2"
                );

                if params[3].as_ref().expect("") == "1" {
                    state.push_value(format!(
                        "s {}",
                        params[1]
                            .as_ref()
                            .expect("encode(pathEvent): cubicBezierSmooth param is none.")
                    ));
                } else {
                    state.push_value(format!(
                        "S {}",
                        params[1]
                            .as_ref()
                            .expect("encode(pathEvent): cubicBezierSmooth param is none.")
                    ));
                }
            }
            "quadraticBezier" => {
                assert_eq!(
                    params.len(),
                    4,
                    "encode(pathEvent): quadraticBezier params != 2"
                );

                if params[3].as_ref().expect("") == "1" {
                    state.push_value(format!(
                        "q {}",
                        params[1]
                            .as_ref()
                            .expect("encode(pathEvent): quadraticBezier param is none.")
                    ));
                } else {
                    state.push_value(format!(
                        "Q {}",
                        params[1]
                            .as_ref()
                            .expect("encode(pathEvent): quadraticBezier param is none.")
                    ));
                }
            }
            "quadraticBezierSmooth" => {
                assert_eq!(
                    params.len(),
                    4,
                    "encode(pathEvent): quadraticBezierSmooth params != 2"
                );

                if params[3].as_ref().expect("") == "1" {
                    state.push_value(format!(
                        "t {}",
                        params[1]
                            .as_ref()
                            .expect("encode(pathEvent): quadraticBezierSmooth param is none.")
                    ));
                } else {
                    state.push_value(format!(
                        "T {}",
                        params[1]
                            .as_ref()
                            .expect("encode(pathEvent): quadraticBezierSmooth param is none.")
                    ));
                }
            }
            "arc" => {
                assert_eq!(params.len(), 4, "encode(pathEvent): arc params != 2");

                if params[3].as_ref().expect("") == "1" {
                    state.push_value(format!(
                        "a {}",
                        params[1]
                            .as_ref()
                            .expect("encode(pathEvent): arc param is none.")
                    ));
                } else {
                    state.push_value(format!(
                        "A {}",
                        params[1]
                            .as_ref()
                            .expect("encode(pathEvent): arc param is none.")
                    ));
                }
            }
            directive => {
                log::error!(target: SVG_WRITE_REPORT,"unknown path directive: {}",directive);
            }
        }
    }
}

#[derive(Debug)]
pub(super) struct BackgroundWriter(
    /// variant
    pub(super) String,
);

impl Encoder for BackgroundWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        match self.0.as_str() {
            "accumulate" => {
                assert_eq!(
                    params.len(),
                    0,
                    "encode(Background): accumulate params != 0"
                );

                state.push_value("accumulate");
            }
            "new" => {
                assert_eq!(params.len(), 2, "encode(Background): new params != 1");

                if let Some(param) = &params[1] {
                    state.push_value(format!("new {}", param));
                } else {
                    state.push_value("new");
                }
            }
            _ => panic!("encode(Background): unknown variant `{}`", self.0),
        }
    }
}

#[derive(Debug)]
pub(super) struct BackgroundNewWriter;

impl Encoder for BackgroundNewWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        assert_eq!(params.len(), 8, "encode(background): new params != 4");

        state.push_value(format!(
            "{},{},{},{}",
            params[1]
                .as_ref()
                .expect("encode(background): new param(x) is none."),
            params[3]
                .as_ref()
                .expect("encode(background): new param(y) is none."),
            params[5]
                .as_ref()
                .expect("encode(background): new param(width) is none."),
            params[7]
                .as_ref()
                .expect("encode(background): new param(height) is none."),
        ));
    }
}

#[derive(Debug)]
pub(super) struct ViewBoxWriter;

impl Encoder for ViewBoxWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        assert_eq!(params.len(), 10, "encode(viewbox): params != 5");

        state.push_insert_attr("viewBox");

        state.push_value(format!(
            "{} {} {} {}",
            params[1]
                .as_ref()
                .expect("encode(viewbox): viewbox param(minx) is none."),
            params[3]
                .as_ref()
                .expect("encode(viewbox): viewbox param(miny) is none."),
            params[5]
                .as_ref()
                .expect("encode(viewbox): viewbox param(width) is none."),
            params[7]
                .as_ref()
                .expect("encode(viewbox): viewbox param(height) is none."),
        ));

        state.insert_attr();

        if let Some(aspect) = &params[9] {
            state.push_insert_attr("preserveAspectRatio");
            state.push_value(aspect);
            state.insert_attr();
        }
    }
}

#[derive(Debug)]
pub(super) struct FeFuncWriter(
    /// variant name.
    pub(super) String,
);

impl Encoder for FeFuncWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        match self.0.as_str() {
            "identity" => {
                assert_eq!(params.len(), 0, "encode(FeFunc): identity params != 0");
                state.push_insert_attr("type");
                state.push_value("identity");
                state.insert_attr();
            }
            "table" => {
                assert_eq!(params.len(), 2, "encode(FeFunc): table params != 1");

                state.push_insert_attr("tableValues");
                state.push_value(
                    params[1]
                        .as_ref()
                        .expect("encode(FeFunc): tableValues is none."),
                );
                state.insert_attr();

                state.push_insert_attr("type");
                state.push_value("table");
                state.insert_attr();
            }
            "discrete" => {
                assert_eq!(params.len(), 2, "encode(FeFunc): discrete params != 1");

                state.push_insert_attr("tableValues");
                state.push_value(
                    params[1]
                        .as_ref()
                        .expect("encode(FeFunc): tableValues is none."),
                );
                state.insert_attr();

                state.push_insert_attr("type");
                state.push_value("discrete");
                state.insert_attr();
            }
            "linear" => {
                assert_eq!(params.len(), 4, "encode(FeFunc): linear params != 2");

                state.push_insert_attr("type");
                state.push_value("linear");
                state.insert_attr();

                state.push_insert_attr("slope");
                state.push_value(params[1].as_ref().expect("encode(FeFunc): slope is none."));
                state.insert_attr();

                state.push_insert_attr("intercept");
                state.push_value(
                    params[3]
                        .as_ref()
                        .expect("encode(FeFunc): intercept is none."),
                );
                state.insert_attr();
            }
            "gamma" => {
                assert_eq!(params.len(), 6, "encode(FeFunc): gamma params != 3");

                state.push_insert_attr("type");
                state.push_value("gamma");
                state.insert_attr();

                state.push_insert_attr("amplitude");
                state.push_value(
                    params[1]
                        .as_ref()
                        .expect("encode(FeFunc): gamma(amplitude) is none."),
                );
                state.insert_attr();

                state.push_insert_attr("exponent");
                state.push_value(
                    params[3]
                        .as_ref()
                        .expect("encode(FeFunc): gamma(exponent) is none."),
                );
                state.insert_attr();

                state.push_insert_attr("offset");
                state.push_value(
                    params[5]
                        .as_ref()
                        .expect("encode(FeFunc): gamma(offset) is none."),
                );
                state.insert_attr();
            }
            _ => {
                panic!("encode(FeFunc): unknown variant {}", self.0);
            }
        }
    }
}

#[derive(Debug)]
pub(super) struct FeFuncXWriter;

impl Encoder for FeFuncXWriter {
    fn encode(&self, _: Vec<Option<String>>, state: &mut WriteState) {
        state.clear();
    }
}

#[derive(Debug)]
pub(super) struct SeqWriter {
    pub(super) len: usize,
    pub(super) sep: String,
}

impl Encoder for SeqWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        assert_eq!(params.len(), self.len, "encode(seq): len != {}", self.len);

        state.push_value(
            params
                .into_iter()
                .filter_map(|v| v)
                .collect::<Vec<_>>()
                .join(&self.sep),
        );
    }
}

#[derive(Debug)]
pub(super) struct FeCompositeOperatorWriter(pub(super) String);

impl Encoder for FeCompositeOperatorWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        match self.0.as_str() {
            "over" | "in" | "out" | "atop" | "xor" => {
                assert_eq!(
                    params.len(),
                    0,
                    "encode(FeCompositeOperator): {} params != 0",
                    self.0
                );
                state.push_value(&self.0);
            }
            "arithmetic" => {
                assert_eq!(
                    params.len(),
                    8,
                    "encode(FeCompositeOperator): arithmetic parrams != 4"
                );

                for i in 0..4 {
                    state.push_insert_attr(&format!("k{}", i + 1));
                    state.push_value(params[i * 2 + 1].as_ref().expect(&format!(
                        "encode(FeCompositeOperator): arithmetic k{} is none.",
                        i + 1,
                    )));
                    state.insert_attr();
                }

                state.push_value(&self.0);
            }
            _ => panic!("encode(FeCompositeOperator): unknown type {}", self.0),
        }
    }
}

#[derive(Debug)]
pub(super) struct FeColorMatrixWriter(
    /// variant name.
    pub(super) String,
);
impl Encoder for FeColorMatrixWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        match self.0.as_str() {
            "matrix" | "saturate" | "hueRotate" => {
                assert_eq!(
                    params.len(),
                    2,
                    "encode(FeColorMatrix): {} params != 1",
                    self.0
                );
                state.push_insert_attr("type");
                state.push_value(&self.0);
                state.insert_attr();

                state.push_value(
                    params[1]
                        .as_ref()
                        .expect(&format!("encode(FeColorMatrix): {} value is none.", self.0)),
                );
            }
            "luminanceToAlpha" => {
                assert_eq!(
                    params.len(),
                    0,
                    "encode(FeColorMatrix): luminanceToAlpha params != 0",
                );

                state.push_insert_attr("type");
                state.push_value(&self.0);
                state.insert_attr();

                state.push_none();
            }
            _ => {
                panic!("encode(FeColorMatrix): unknown variant {}.", self.0)
            }
        }
    }
}

#[derive(Debug)]
pub(super) struct PreserveAspectRatioWriter(pub(super) String);

impl Encoder for PreserveAspectRatioWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        match self.0.as_str() {
            "none" => {
                assert_eq!(
                    params.len(),
                    1,
                    "encode(PreserveAspectRatio): `none` params != 0",
                );

                state.push_value("none");
            }
            _ => {
                assert_eq!(
                    params.len(),
                    2,
                    "encode(PreserveAspectRatio): `{}` params != 1",
                    self.0
                );

                state.push_value(format!(
                    "{} {}",
                    self.0,
                    params[1]
                        .as_ref()
                        .expect("encode(PreserveAspectRatio): `meet_or_slice` is none.")
                ));
            }
        }
    }
}

#[derive(Debug)]
pub(super) struct ArcWriter;

impl Encoder for ArcWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        assert_eq!(params.len(), 12, "encode(arc): params len != 6");

        state.push_value(format!(
            "{},{} {} {} {} {}",
            params[1].as_ref().expect("encode(arc): rx is none."),
            params[3].as_ref().expect("encode(arc): ry is none."),
            params[5]
                .as_ref()
                .expect("encode(arc): x_rotation is none."),
            params[7].as_ref().expect("encode(arc): large_arc is none."),
            params[9].as_ref().expect("encode(arc): sweep is none."),
            params[11].as_ref().expect("encode(arc): to is none."),
        ));
    }
}

#[derive(Debug)]
pub(super) struct CubicBezierWriter;

impl Encoder for CubicBezierWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        assert_eq!(params.len(), 6, "encode(cubicBezier): params len != 3");

        state.push_value(format!(
            "{} {} {}",
            params[1]
                .as_ref()
                .expect("encode(cubicBezier): ctrl1 is none."),
            params[3]
                .as_ref()
                .expect("encode(cubicBezier): ctrl2 is none."),
            params[5]
                .as_ref()
                .expect("encode(cubicBezier): to is none."),
        ));
    }
}

#[derive(Debug)]
pub(super) struct CubicBezierSmoothWriter;

impl Encoder for CubicBezierSmoothWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        assert_eq!(
            params.len(),
            4,
            "encode(cubicBezierSmooth): params len != 2"
        );

        state.push_value(format!(
            "{} {}",
            params[1]
                .as_ref()
                .expect("encode(cubicBezierSmooth): ctrl2 is none."),
            params[3]
                .as_ref()
                .expect("encode(cubicBezierSmooth): to is none."),
        ));
    }
}

#[derive(Debug)]
pub(super) struct QuadraticBezierWriter;

impl Encoder for QuadraticBezierWriter {
    fn encode(&self, params: Vec<Option<String>>, state: &mut WriteState) {
        assert_eq!(params.len(), 4, "encode(quadraticBezier): params len != 2");

        state.push_value(format!(
            "{} {}",
            params[1]
                .as_ref()
                .expect("encode(quadraticBezier): ctrl is none."),
            params[3]
                .as_ref()
                .expect("encode(quadraticBezier): to is none."),
        ));
    }
}
