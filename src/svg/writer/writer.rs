use mlang_rs::rt::serde::ser::{Serialize, SerializeNode, SerializeSeq, Serializer};
use xml_dom::level2::{
    ext::{get_implementation_ext, DocumentDecl, XmlDecl},
    Document, Element, Node, RefNode,
};

use crate::opcode::{Leaf, Opcode};

use super::{state::*, Result, WritingError};

struct SvgWriter {
    /// svg document.
    document: RefNode,
    /// current write state.
    state: Vec<WriteState>,
    /// xml building stack.
    building_stack: Vec<RefNode>,
    /// Defs xml nodes.
    defs: Vec<RefNode>,
}

impl SvgWriter {
    fn new() -> Self {
        let implementation = get_implementation_ext();

        let mut document = implementation
            .create_document(None, Some("svg"), None)
            .unwrap();

        let decl = XmlDecl::new(
            xml_dom::level2::ext::XmlVersion::V10,
            Some("utf-8".to_string()),
            Some(true),
        );

        document.set_xml_declaration(decl).unwrap();

        Self {
            document,
            state: Default::default(),
            building_stack: Default::default(),
            defs: Default::default(),
        }
    }

    /// Convert self into svg document.
    fn into_bytes(self) -> Vec<u8> {
        assert_eq!(
            self.building_stack.len(),
            0,
            "SvgWriter: building stack is not empty."
        );

        self.document.to_string().as_bytes().to_vec()
    }
}

impl<'a> Serializer for &'a mut SvgWriter {
    type Error = WritingError;

    type SerializeNode = Self;

    type SerializeSeq = Self;

    fn serialize_el(
        self,
        _: usize,
        name: &str,
        _: usize,
    ) -> std::result::Result<Self::SerializeNode, Self::Error> {
        self.state.push(WriteState::default());

        if self.building_stack.is_empty() {
            if name != "svg" {
                return Err(WritingError::Root);
            }

            self.building_stack
                .push(self.document.document_element().unwrap());
        } else {
            self.building_stack
                .push(self.document.create_element(name)?);
        }

        Ok(self)
    }

    fn serialize_leaf(
        self,
        _: usize,
        name: &str,
        _: usize,
    ) -> std::result::Result<Self::SerializeNode, Self::Error> {
        match name {
            "characters" => {
                let state = self.state.last_mut().unwrap();
                let node = self
                    .building_stack
                    .last_mut()
                    .expect("serialize `characters`")
                    .clone();

                state.push_encoder(CharactersWriter {
                    document: self.document.clone(),
                    node,
                });
            }
            "feFuncR" | "feFuncG" | "feFuncB" | "feFuncA" => {
                self.state.push(WriteState::default());
                let state = self.state.last_mut().unwrap();
                state.push_encoder(FeFuncXWriter);

                self.building_stack
                    .push(self.document.create_element(name)?);
            }
            _ => {
                self.state.push(WriteState::default());
                self.building_stack
                    .push(self.document.create_element(name)?);
            }
        }

        Ok(self)
    }

    fn serialize_attr(
        self,
        _: usize,
        name: &str,
        _: usize,
    ) -> std::result::Result<Self::SerializeNode, Self::Error> {
        let state = self
            .state
            .last_mut()
            .expect("serialize_attr: state is none");

        match name {
            "viewBox" => {
                state.push_encoder(ViewBoxWriter);
            }
            _ => {}
        }

        Ok(self)
    }

    fn serialize_data(
        self,
        _: usize,
        name: &str,
        _: usize,
    ) -> std::result::Result<Self::SerializeNode, Self::Error> {
        let state = self.state.last_mut().unwrap();

        match name {
            "rgb" => {
                state.push_encoder(RgbWriter);
            }
            "funcIri" => {
                state.push_encoder(FuncIriWriter);
            }
            "point" => {
                state.push_encoder(PointWriter);
            }
            "percent" => {
                state.push_encoder(PercentWriter);
            }
            "numberOptNumber" => {
                state.push_encoder(NumberOptNumberWriter);
            }
            "backgroundNew" => {
                state.push_encoder(BackgroundNewWriter);
            }
            "arc" => {
                state.push_encoder(ArcWriter);
            }
            "cubicBezier" => {
                state.push_encoder(CubicBezierWriter);
            }
            "cubicBezierSmooth" => {
                state.push_encoder(CubicBezierSmoothWriter);
            }
            "quadraticBezier" => {
                state.push_encoder(QuadraticBezierWriter);
            }
            _ => {
                panic!("SvgWriter: unsupport data type `{}`", name);
            }
        }

        Ok(self)
    }

    fn serialize_enum(
        self,
        _: usize,
        name: &str,
        variant: &str,
        _: usize,
        fields: usize,
    ) -> std::result::Result<Self::SerializeNode, Self::Error> {
        let state = self.state.last_mut().unwrap();

        match name {
            "feFunc" => {
                state.push_encoder(FeFuncWriter(variant.to_string()));
            }
            "angle" => {
                state.push_encoder(AngleWriter(variant.to_string()));
            }
            "pathEvent" => {
                state.push_encoder(PathEventWriter(variant.to_string()));
            }
            "transform" => {
                state.push_encoder(TransformWriter(variant.to_string()));
            }
            "length" => {
                state.push_encoder(LengthWriter(variant.to_string()));
            }
            "iri" => {
                state.push_encoder(IriWriter(variant.to_string()));
            }
            "preserveAspectRatio" => {
                state.push_encoder(PreserveAspectRatioWriter(variant.to_string()));
            }
            "feColorMatrixValues" => {
                state.push_encoder(FeColorMatrixWriter(variant.to_string()));
            }
            "feCompositeOperator" => {
                state.push_encoder(FeCompositeOperatorWriter(variant.to_string()));
            }
            "background" => {
                state.push_encoder(BackgroundWriter(variant.to_string()));
            }
            "stroke-linejoin" => {
                state.push_encoder(StrokeLineJoinWriter(variant.to_string()));
            }
            "color" => {
                state.push_encoder(ColorWriter(variant.to_string()));
            }
            _ => {
                state.push_encoder(EnumWriter {
                    ty: name.to_string(),
                    variant: variant.to_string(),
                    fields,
                });
            }
        }

        Ok(self)
    }

    fn serialize_seq(self, len: usize) -> std::result::Result<Self::SerializeSeq, Self::Error> {
        let state = self.state.last_mut().unwrap();

        state.push_encoder(SeqWriter {
            len,
            sep: " ".to_string(),
        });

        Ok(self)
    }

    fn serialize_bool(self, value: bool) -> std::result::Result<(), Self::Error> {
        self.serialize_string(&format!("{}", if value { 1 } else { 0 }))
    }

    fn serialize_string(self, value: &str) -> std::result::Result<(), Self::Error> {
        let state = self.state.last_mut().unwrap();

        state.push_value(value);

        Ok(())
    }

    fn serialize_byte(self, value: i8) -> std::result::Result<(), Self::Error> {
        self.serialize_string(&format!("{}", value))
    }

    fn serialize_ubyte(self, value: u8) -> std::result::Result<(), Self::Error> {
        self.serialize_string(&format!("{}", value))
    }

    fn serialize_short(self, value: i16) -> std::result::Result<(), Self::Error> {
        self.serialize_string(&format!("{}", value))
    }

    fn serialize_ushort(self, value: u16) -> std::result::Result<(), Self::Error> {
        self.serialize_string(&format!("{}", value))
    }

    fn serialize_int(self, value: i32) -> std::result::Result<(), Self::Error> {
        self.serialize_string(&format!("{}", value))
    }

    fn serialize_uint(self, value: u32) -> std::result::Result<(), Self::Error> {
        self.serialize_string(&format!("{}", value))
    }

    fn serialize_long(self, value: i64) -> std::result::Result<(), Self::Error> {
        self.serialize_string(&format!("{}", value))
    }

    fn serialize_ulong(self, value: u64) -> std::result::Result<(), Self::Error> {
        self.serialize_string(&format!("{}", value))
    }

    fn serialize_float(self, value: f32) -> std::result::Result<(), Self::Error> {
        self.serialize_string(&format!("{}", value))
    }

    fn serialize_double(self, value: f64) -> std::result::Result<(), Self::Error> {
        self.serialize_string(&format!("{}", value))
    }

    fn serialize_none(self) -> std::result::Result<(), Self::Error> {
        let state = self.state.last_mut().unwrap();

        state.push_none();
        Ok(())
    }

    fn serialize_variable(
        self,
        _: &mlang_rs::rt::opcode::Path,
        _: &mlang_rs::rt::opcode::Target,
    ) -> std::result::Result<(), Self::Error> {
        return Err(WritingError::Variable);
    }

    fn serialize_pop(self) -> std::result::Result<(), Self::Error> {
        let mut svg_node = self
            .building_stack
            .pop()
            .expect("serialize_pop: null building_stack");

        let tag_name = svg_node.tag_name();

        let state = self
            .state
            .pop()
            .expect(&format!("({}) serialize_pop: state is none.", tag_name));

        state.write(&mut svg_node)?;

        if svg_node.has_attribute("id") {
            self.defs.push(svg_node);
            return Ok(());
        }

        if let Some(last) = self.building_stack.last_mut() {
            last.append_child(svg_node)?;
        } else {
            // is document root node.
            svg_node.set_attribute("xmlns", "http://www.w3.org/2000/svg")?;
            svg_node.set_attribute("version", "1.1")?;
            svg_node.set_attribute("xmlns:xlink", "http://www.w3.org/1999/xlink")?;

            if self.defs.len() > 0 {
                let mut defs = self.document.create_element("defs")?;

                for node in self.defs.drain(..) {
                    defs.append_child(node)?;
                }

                svg_node.insert_before(defs, svg_node.first_child())?;
            }
        }

        Ok(())
    }
}

impl<'a> SerializeSeq for &'a mut SvgWriter {
    type Error = WritingError;

    fn next_item<T>(&mut self, value: &T) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(&mut **self)?;

        Ok(())
    }

    fn finish(self) -> std::result::Result<(), Self::Error> {
        let state = self.state.last_mut().unwrap();

        state.encode()
    }
}

impl<'a> SerializeNode for &'a mut SvgWriter {
    type Error = WritingError;

    fn serialize_field<T>(
        &mut self,
        index: usize,
        name: Option<&str>,
        value: &T,
    ) -> std::result::Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let state = self.state.last_mut().unwrap();

        if let Some(name) = name {
            state.push_insert_attr(name);
        } else {
            state.push_insert_attr(index.to_string());
        }

        value.serialize(&mut **self)?;

        Ok(())
    }

    fn finish(self) -> std::result::Result<(), Self::Error> {
        let state = self
            .state
            .last_mut()
            .expect("SerializeNode(finish): state is none.");

        state.encode()
    }
}

/// Convert vglang opcodes into svg image.
pub fn to_svg(opcodes: impl AsRef<[Opcode]>) -> Result<Vec<u8>> {
    let mut writer = SvgWriter::new();
    let mut attrs = vec![];

    for opcode in opcodes.as_ref() {
        match opcode {
            Opcode::Apply(_) => {
                attrs.push(opcode);
            }
            Opcode::Leaf(leaf) => {
                opcode.serialize(&mut writer)?;

                for attr in attrs.drain(..) {
                    attr.serialize(&mut writer)?;
                }
                match leaf {
                    Leaf::Characters(_) => {}
                    _ => {
                        writer.serialize_pop()?;
                    }
                }
            }
            Opcode::Element(_) => {
                opcode.serialize(&mut writer)?;

                for attr in attrs.drain(..) {
                    attr.serialize(&mut writer)?;
                }
            }
            _ => opcode.serialize(&mut writer)?,
        }
    }

    Ok(writer.into_bytes())
}
