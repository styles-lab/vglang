use std::io::Cursor;

use mlang_rs::rt::serde::de::{AttrsNodeAccess, Deserialize, Deserializer, NodeAccess, SeqAccess};
use xml_dom::{
    level2::{Element, Node, RefNode},
    parser::read_reader,
};

use crate::{
    opcode::Opcode,
    svg::reader::{
        state::{
            AngleListDecoder, CharactersDecoder, ColorDecoder, FontFamilyListDecoder, IriDecoder,
            LengthDecoder, LengthListDecoder, PaintDecoder, PathEventsDecoder, PointDecoder,
            ReadingCode, StrokeLineJoinDecoder, TransformListDecoder, VariantDecoder,
            ViewBoxDecoder,
        },
        SVG_READ_REPORT,
    },
};

use super::{
    state::{
        AngleDecoder, FeColorMatrixDecoder, FeFuncDecoder, FeInDecoder, FuncIriDecoder,
        NumberOptNumberDecoder, PointListDecoder, ReadingState,
    },
    ReadingError, Result,
};

enum Reading {
    Normal(RefNode),
    ChildOfDefs(RefNode),
    /// only for vglang `el` node.
    Handled,
}

#[derive(Default)]
struct SvgReader {
    /// a stack of processing xml nodes.
    reading_stack: Vec<Reading>,
    /// current deserializing data.
    state: ReadingState,
}

impl SvgReader {
    pub fn parse(xml: impl AsRef<[u8]>) -> Result<Self> {
        let document = read_reader(Cursor::new(xml))?;

        let root = document
            .child_nodes()
            .into_iter()
            .find(|node| node.tag_name() == "svg")
            .ok_or(ReadingError::LoadSvgElement)?;

        Ok(SvgReader {
            reading_stack: vec![Reading::Normal(root)],
            state: Default::default(),
        })
    }
}

impl<'a> Deserializer for &'a mut SvgReader {
    type Error = ReadingError;

    fn deserialize_opcode<V>(
        self,
        visitor: V,
    ) -> std::result::Result<Option<Vec<V::Value>>, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        loop {
            let (node, child_of_defs) = match self.reading_stack.pop() {
                Some(Reading::Normal(node)) => (node, false),
                Some(Reading::ChildOfDefs(node)) => (node, true),
                Some(Reading::Handled) => {
                    log::trace!(target: SVG_READ_REPORT, "deserialize pop");
                    return visitor.visit_pop().map(|v| Some(vec![v]));
                }
                None => return Ok(None),
            };

            let tag_name = node.tag_name();

            let child_of_defs = child_of_defs || tag_name == "defs";

            let mut children = node
                .child_nodes()
                .into_iter()
                .rev()
                .map(|node| {
                    if child_of_defs {
                        Reading::ChildOfDefs(node)
                    } else {
                        Reading::Normal(node)
                    }
                })
                .collect::<Vec<_>>();

            // skip deserializing `defs` elem.
            if tag_name == "defs" {
                self.reading_stack.append(&mut children);
                continue;
            }

            self.state = ReadingState::from_xml_node(&node, child_of_defs);

            let opcode_name = self.state.opcode_name().to_string();

            if visitor.is_element(&opcode_name) {
                self.reading_stack.push(Reading::Handled);
                self.state.push(ReadingCode::Elem(opcode_name.clone()));
            } else if !visitor.is_leaf(&opcode_name) {
                log::debug!("skip unknown elm {}", self.state.opcode_name());
                continue;
            } else {
                self.state.push(ReadingCode::Leaf(opcode_name.clone()));
            }

            self.reading_stack.append(&mut children);

            return visitor
                .visit_opcode_with_attrs(&opcode_name, &mut *self)
                .map(|opcodes| Some(opcodes));
        }
    }

    fn deserialize_element<V>(
        self,
        _: usize,
        name: &str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        let v = visitor.visit_node(&mut *self)?;

        assert_eq!(self.state.pop(), Some(ReadingCode::Elem(name.to_string())));

        Ok(v)
    }

    fn deserialize_leaf<V>(
        self,
        _: usize,
        name: &str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        match name {
            "characters" => {
                self.state.decode::<CharactersDecoder>()?;
            }
            _ => {}
        }

        let value = visitor.visit_node(&mut *self)?;

        assert_eq!(self.state.pop(), Some(ReadingCode::Leaf(name.to_string())));

        Ok(value)
    }

    fn deserialize_attr<V>(
        self,
        _: usize,
        name: &str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        match name {
            "viewBox" => {
                self.state.decode::<ViewBoxDecoder>()?;
                assert_eq!(self.state.pop(), Some(ReadingCode::Attr(name.to_string())));
            }
            _ => {}
        }

        visitor.visit_node(&mut *self)
    }

    fn deserialize_data<V>(
        self,
        _: usize,
        name: &str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        match self.state.top() {
            Some(ReadingCode::Data(v)) => {
                assert_eq!(name, v);
            }
            Some(ReadingCode::SeqStart) => {
                self.state.pop();

                match name {
                    "point" => {
                        self.state.decode::<PointListDecoder>()?;
                    }
                    _ => {
                        panic!("unhandle data({}) list", name);
                    }
                }

                assert_eq!(self.state.pop(), Some(ReadingCode::SeqStart));
            }
            _ => match name {
                "funcIri" => {
                    self.state.decode::<FuncIriDecoder>()?;
                }
                "point" => {
                    self.state.decode::<PointDecoder>()?;
                }
                "numberOptNumber" => {
                    self.state.decode::<NumberOptNumberDecoder>()?;
                }
                "backgroundNew" => {
                    self.state
                        .push(ReadingCode::Data("backgroundNew".to_string()));
                }
                _ => {
                    panic!("unhandle data({})", name);
                }
            },
        }

        assert_eq!(self.state.pop(), Some(ReadingCode::Data(name.to_string())));

        visitor.visit_node(&mut *self)
    }

    fn deserialize_enum<V>(
        self,
        _: usize,
        name: &str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        match self.state.top() {
            Some(ReadingCode::Variant(_)) => {}
            Some(ReadingCode::SeqStart) => {
                self.state.pop();

                match name {
                    "pathEvent" => {
                        self.state.decode::<PathEventsDecoder>()?;
                    }
                    "transform" => {
                        self.state.decode::<TransformListDecoder>()?;
                    }
                    "fontFamily" => {
                        self.state.decode::<FontFamilyListDecoder>()?;
                    }
                    "angle" => {
                        self.state.decode::<AngleListDecoder>()?;
                    }
                    "length" => {
                        self.state.decode::<LengthListDecoder>()?;
                    }
                    _ => {
                        panic!("unhandle variant({})", name);
                    }
                }

                assert_eq!(self.state.pop(), Some(ReadingCode::SeqStart));
            }
            _ => match name {
                "length" => {
                    self.state.decode::<LengthDecoder>()?;
                }
                "paint" => {
                    self.state.decode::<PaintDecoder>()?;
                }
                "color" => {
                    self.state.decode::<ColorDecoder>()?;
                }
                "stroke-linejoin" => {
                    self.state.decode::<StrokeLineJoinDecoder>()?;
                }
                "iri" => {
                    self.state.decode::<IriDecoder>()?;
                }
                "feColorMatrixValues" => {
                    self.state.decode::<FeColorMatrixDecoder>()?;
                }
                "angle" => {
                    self.state.decode::<AngleDecoder>()?;
                }
                "transform" => {
                    self.state.decode::<TransformListDecoder>()?;
                }
                "feFunc" => {
                    self.state.decode::<FeFuncDecoder>()?;
                }
                "feIn" => {
                    self.state.decode::<FeInDecoder>()?;
                }
                _ => {
                    self.state.decode::<VariantDecoder>()?;
                }
            },
        }

        match self.state.pop() {
            Some(ReadingCode::Variant(variant)) => visitor.visit_enum_with(&variant, &mut *self),
            code => {
                log::error!(
                    target: SVG_READ_REPORT,
                    "{:?} unhandle variant({}) = {:?}", self.state.codes, name,code
                );
                panic!("inner error: unhandle variant(`{}`)", name);
            }
        }
    }

    fn deserialize_seq<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        match self.state.pop() {
            Some(ReadingCode::SeqStart) => {}
            Some(ReadingCode::None) => return Err(ReadingError::None),
            Some(code) => {
                self.state.push(code);
                self.state.push_seq_start();
            }
            None => return Err(ReadingError::None),
        }

        let v = visitor.visit_seq(&mut *self)?;

        assert_eq!(self.state.pop(), Some(ReadingCode::SeqEnd));

        Ok(v)
    }

    fn deserialize_option<V>(self, visitor: V) -> std::result::Result<Option<V::Value>, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        if self.state.pop_none() {
            return Ok(None);
        }

        match visitor.visit_some(self) {
            Ok(v) => Ok(Some(v)),
            Err(ReadingError::None) => Ok(None),
            Err(err) => Err(err),
        }
    }

    fn deserialize_variable<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        visitor.visit_constant(self)
    }

    fn deserialize_string<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        visitor.visit_string(&self.state.pop_value()?)
    }

    fn deserialize_bool<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_byte<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_ubyte<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_short<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_ushort<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_int<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_uint<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_long<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_ulong<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_float<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_double<V>(self, visitor: V) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        self.deserialize_string(visitor)
    }
}

impl<'a> NodeAccess for &'a mut SvgReader {
    type Error = ReadingError;

    fn deserialize_field<T>(
        &mut self,
        _: &str,
        _: usize,
        field_name: Option<&str>,
    ) -> std::result::Result<T::Value, Self::Error>
    where
        T: mlang_rs::rt::serde::de::Deserialize,
    {
        match self.state.top() {
            Some(ReadingCode::Field { name: _, index: _ }) => {
                self.state.pop();
            }
            _ => {
                if let Some(name) = field_name {
                    self.state.load(name);
                }
            }
        }

        T::deserialize(&mut **self)
    }
}

impl<'a> SeqAccess for &'a mut SvgReader {
    type Error = ReadingError;

    fn next_item<T>(&mut self) -> std::result::Result<Option<T::Value>, Self::Error>
    where
        T: mlang_rs::rt::serde::de::Deserialize,
    {
        match self.state.top() {
            Some(ReadingCode::SeqEnd) => {
                return Ok(None);
            }
            _ => {}
        }

        T::deserialize(&mut **self).map(|v| Some(v))
    }
}

impl<'a> AttrsNodeAccess for &'a mut SvgReader {
    type Error = ReadingError;

    fn attrs(&self) -> impl Iterator<Item = &str> {
        self.state.attrs()
    }

    fn deserialize_attr<V>(
        &mut self,
        name: &str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        visitor.visit_opcode_with(name, &mut **self)
    }

    fn deserialize_node<V>(
        self,
        name: &str,
        visitor: V,
    ) -> std::result::Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        visitor.visit_opcode_with(name, self)
    }
}

/// Deserialize `vglang` opcodes from svg format.
pub fn from_svg(xml: impl AsRef<[u8]>) -> Result<Vec<Opcode>> {
    let mut reader = SvgReader::parse(xml)?;

    let mut opcodes = vec![];

    while let Some(mut append) = Opcode::deserialize(&mut reader)? {
        opcodes.append(&mut append);
    }

    Ok(opcodes)
}
