use std::str::{from_utf8, Utf8Error};

use bytes::{Buf, Bytes, TryGetError};
use mlang_rs::rt::{
    opcode::Target,
    serde::de::{Deserialize, Deserializer, NodeAccess, SeqAccess},
};

use crate::{
    binary::keyword::Keyword,
    opcode::{variable, Opcode},
};

/// Error type returns by this module.
#[derive(Debug, thiserror::Error)]
pub enum ReadError {
    #[error(transparent)]
    DeEerr(#[from] mlang_rs::rt::serde::de::Error),

    #[error("incomplete {0}: {1}")]
    Incomplete(ReadKind, TryGetError),

    #[error("expect {0}, got {1}")]
    Mismatch(u8, u8),

    #[error("read {0} error: {1}")]
    Utf8Error(ReadKind, Utf8Error),

    #[error("unknown variable target: {0}")]
    UnknownTarget(u8),

    #[error("expect opcode type_id, but got keyword {0:?}")]
    Keyword(Keyword),
}
/// Error type returns by this module.
#[derive(Debug, thiserror::Error)]
pub enum ReadKind {
    #[error("enum({0})")]
    Enum(String),
    #[error("option")]
    Option,
    #[error("variable")]
    Variable,
    #[error("{0:?}")]
    Keyword(Keyword),
}

const BINARY_READ_REPORT: &str = "BINARY_READ_REPORT";

struct BinaryReader {
    buf: Bytes,
    /// sequence reading count.
    counters: Vec<usize>,
}

impl<'a> Deserializer for &'a mut BinaryReader {
    type Error = ReadError;

    fn deserialize_opcode<V>(self, visitor: V) -> Result<Option<Vec<V::Value>>, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        if self.buf.is_empty() {
            return Ok(None);
        }

        let type_id = self.buf.get_u8();

        log::trace!(target: BINARY_READ_REPORT, "deserialize_opcode, {}",type_id);

        match Keyword::try_from(type_id) {
            Ok(v) => {
                if v == Keyword::Pop {
                    return visitor.visit_pop().map(|v| Some(vec![v]));
                }
                log::error!(target: BINARY_READ_REPORT, "unexpect keyword, {:?}",v);
                return Err(ReadError::Keyword(v));
            }
            Err(type_id) => {
                log::trace!(target: BINARY_READ_REPORT, "read opcode, {}",type_id);
                let opcode = visitor.visit_opcode(type_id, self)?;
                Ok(Some(vec![opcode]))
            }
        }
    }

    fn deserialize_element<V>(
        self,
        _: usize,
        name: &str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        log::trace!(target: BINARY_READ_REPORT, "read el({})",name);
        visitor.visit_node(self)
    }

    fn deserialize_leaf<V>(self, _: usize, name: &str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        log::trace!(target: BINARY_READ_REPORT, "read leaf({})",name);
        visitor.visit_node(self)
    }

    fn deserialize_attr<V>(self, _: usize, name: &str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        log::trace!(target: BINARY_READ_REPORT, "read attr({})",name);
        visitor.visit_node(self)
    }

    fn deserialize_data<V>(self, _: usize, name: &str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        log::trace!(target: BINARY_READ_REPORT, "read data({})",name);
        visitor.visit_node(self)
    }

    fn deserialize_enum<V>(self, _: usize, name: &str, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        let variant_index = self
            .buf
            .try_get_u8()
            .map_err(|err| ReadError::Incomplete(ReadKind::Enum(name.to_string()), err))?
            as usize;

        log::trace!(target: BINARY_READ_REPORT, "read enum({}:{})",name,variant_index);

        visitor.visit_enum(variant_index, self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        let ty = self
            .buf
            .try_get_u8()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Seq), err))?;

        if u8::from(Keyword::Seq) != ty {
            return Err(ReadError::Mismatch(Keyword::Seq.into(), ty));
        }

        let counter = self
            .buf
            .try_get_u16()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Seq), err))?
            as usize;

        log::trace!(target: BINARY_READ_REPORT, "read seq({})",counter);

        self.counters.push(counter);

        visitor.visit_seq(self)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<Option<V::Value>, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        if self.buf.is_empty() {
            return Err(ReadError::Incomplete(
                ReadKind::Option,
                TryGetError {
                    requested: 1,
                    available: 0,
                },
            ));
        }

        let ty = self.buf[0];

        if ty == Keyword::None.into() {
            self.buf.get_u8();
            return Ok(None);
        }

        visitor.visit_some(self).map(|v| Some(v))
    }

    fn deserialize_variable<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        if self.buf.is_empty() {
            return Err(ReadError::Incomplete(
                ReadKind::Variable,
                TryGetError {
                    requested: 1,
                    available: 0,
                },
            ));
        }

        let ty = self.buf[0];

        if u8::from(Keyword::Path) == ty {
            self.buf.get_u8();

            let ty = self
                .buf
                .try_get_u8()
                .map_err(|err| ReadError::Incomplete(ReadKind::Variable, err))?;

            let path = if u8::from(Keyword::String) == ty {
                let len = self
                    .buf
                    .try_get_u16()
                    .map_err(|err| ReadError::Incomplete(ReadKind::Variable, err))?
                    as usize;

                if self.buf.len() < len {
                    return Err(ReadError::Incomplete(
                        ReadKind::Variable,
                        TryGetError {
                            requested: len,
                            available: self.buf.len(),
                        },
                    ));
                }

                let path = self.buf.copy_to_bytes(len);

                variable::Path::Named(
                    from_utf8(&path)
                        .map_err(|err| ReadError::Utf8Error(ReadKind::Variable, err))?
                        .to_string(),
                )
            } else if u8::from(Keyword::Ushort) == ty {
                let index = self
                    .buf
                    .try_get_u16()
                    .map_err(|err| ReadError::Incomplete(ReadKind::Variable, err))?
                    as usize;

                variable::Path::Index(index)
            } else {
                return Err(ReadError::Mismatch(Keyword::String.into(), ty));
            };

            let ty = self
                .buf
                .try_get_u8()
                .map_err(|err| ReadError::Incomplete(ReadKind::Variable, err))?;

            if u8::from(Keyword::Target) != ty {
                return Err(ReadError::Mismatch(Keyword::Target.into(), ty));
            }

            let target = self
                .buf
                .try_get_u8()
                .map_err(|err| ReadError::Incomplete(ReadKind::Variable, err))?;

            let target = match target {
                0 => Target::Register,
                1 => Target::ForeachItem,
                2 => Target::ForeachIndex,
                3 => Target::Range,
                _ => return Err(ReadError::UnknownTarget(target)),
            };

            return visitor.visit_variable(path, target);
        }

        visitor.visit_constant(self)
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        let ty = self
            .buf
            .try_get_u8()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::String), err))?;

        if u8::from(Keyword::String) != ty {
            return Err(ReadError::Mismatch(Keyword::String.into(), ty));
        }

        let len = self
            .buf
            .try_get_u16()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::String), err))?
            as usize;

        if self.buf.len() < len {
            return Err(ReadError::Incomplete(
                ReadKind::Keyword(Keyword::String),
                TryGetError {
                    requested: len,
                    available: self.buf.len(),
                },
            ));
        }

        let buf = self.buf.copy_to_bytes(len);

        let value = from_utf8(&buf)
            .map_err(|err| ReadError::Utf8Error(ReadKind::Keyword(Keyword::String), err))?;

        visitor.visit_string(value)
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        let ty = self
            .buf
            .try_get_u8()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::String), err))?;

        if u8::from(Keyword::Bool) != ty {
            return Err(ReadError::Mismatch(Keyword::Bool.into(), ty));
        }

        let value = self
            .buf
            .try_get_u8()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Bool), err))?;

        visitor.visit_bool(if value == 0 { false } else { true })
    }

    fn deserialize_byte<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        let ty = self
            .buf
            .try_get_u8()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Byte), err))?;

        if u8::from(Keyword::Byte) != ty {
            return Err(ReadError::Mismatch(Keyword::Byte.into(), ty));
        }

        let value = self
            .buf
            .try_get_i8()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Byte), err))?;

        visitor.visit_byte(value)
    }

    fn deserialize_ubyte<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        let ty = self
            .buf
            .try_get_u8()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Ubyte), err))?;

        if u8::from(Keyword::Ubyte) != ty {
            return Err(ReadError::Mismatch(Keyword::Ubyte.into(), ty));
        }

        let value = self
            .buf
            .try_get_u8()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Ubyte), err))?;

        visitor.visit_ubyte(value)
    }

    fn deserialize_short<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        let ty = self
            .buf
            .try_get_u8()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Short), err))?;

        if u8::from(Keyword::Short) != ty {
            return Err(ReadError::Mismatch(Keyword::Short.into(), ty));
        }

        let value = self
            .buf
            .try_get_i16()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Short), err))?;

        visitor.visit_short(value)
    }

    fn deserialize_ushort<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        let ty = self
            .buf
            .try_get_u8()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Ushort), err))?;

        if u8::from(Keyword::Ushort) != ty {
            return Err(ReadError::Mismatch(Keyword::Ushort.into(), ty));
        }

        let value = self
            .buf
            .try_get_u16()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Ushort), err))?;

        visitor.visit_ushort(value)
    }

    fn deserialize_int<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        let ty = self
            .buf
            .try_get_u8()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Int), err))?;

        if u8::from(Keyword::Int) != ty {
            return Err(ReadError::Mismatch(Keyword::Int.into(), ty));
        }

        let value = self
            .buf
            .try_get_i32()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Int), err))?;

        visitor.visit_int(value)
    }

    fn deserialize_uint<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        let ty = self
            .buf
            .try_get_u8()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Uint), err))?;

        if u8::from(Keyword::Uint) != ty {
            return Err(ReadError::Mismatch(Keyword::Uint.into(), ty));
        }

        let value = self
            .buf
            .try_get_u32()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Uint), err))?;

        visitor.visit_uint(value)
    }

    fn deserialize_long<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        let ty = self
            .buf
            .try_get_u8()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Long), err))?;

        if u8::from(Keyword::Long) != ty {
            return Err(ReadError::Mismatch(Keyword::Long.into(), ty));
        }

        let value = self
            .buf
            .try_get_i64()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Long), err))?;

        visitor.visit_long(value)
    }

    fn deserialize_ulong<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        let ty = self
            .buf
            .try_get_u8()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Ulong), err))?;

        if u8::from(Keyword::Ulong) != ty {
            return Err(ReadError::Mismatch(Keyword::Ulong.into(), ty));
        }

        let value = self
            .buf
            .try_get_u64()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Ulong), err))?;

        visitor.visit_ulong(value)
    }

    fn deserialize_float<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        let ty = self
            .buf
            .try_get_u8()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Float), err))?;

        if u8::from(Keyword::Float) != ty {
            return Err(ReadError::Mismatch(Keyword::Float.into(), ty));
        }

        let value = self
            .buf
            .try_get_f32()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Float), err))?;

        visitor.visit_float(value)
    }

    fn deserialize_double<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        let ty = self
            .buf
            .try_get_u8()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Double), err))?;

        if u8::from(Keyword::Double) != ty {
            return Err(ReadError::Mismatch(Keyword::Double.into(), ty));
        }

        let value = self
            .buf
            .try_get_f64()
            .map_err(|err| ReadError::Incomplete(ReadKind::Keyword(Keyword::Double), err))?;

        visitor.visit_double(value)
    }
}

impl<'a> NodeAccess for &'a mut BinaryReader {
    type Error = ReadError;

    fn deserialize_field<T>(
        &mut self,
        ty: &str,
        index: usize,
        field: Option<&str>,
    ) -> Result<T::Value, Self::Error>
    where
        T: mlang_rs::rt::serde::de::Deserialize,
    {
        if let Some(field) = field {
            log::trace!(target: BINARY_READ_REPORT, "read `{}` field({})",ty, field);
        } else {
            log::trace!(target: BINARY_READ_REPORT, "read `{}` field({})",ty, index);
        }

        T::deserialize(&mut **self)
    }
}

impl<'a> SeqAccess for &'a mut BinaryReader {
    type Error = ReadError;

    fn next_item<T>(&mut self) -> Result<Option<T::Value>, Self::Error>
    where
        T: mlang_rs::rt::serde::de::Deserialize,
    {
        if *self.counters.last().expect("check counter") == 0 {
            self.counters.pop();
            return Ok(None);
        }

        *self.counters.last_mut().unwrap() -= 1;

        T::deserialize(&mut **self).map(|v| Some(v))
    }
}

/// Decode opcodes from binary format.
pub fn from_binary(data: impl ToOwned<Owned = Vec<u8>>) -> Result<Vec<Opcode>, ReadError> {
    let mut reader = BinaryReader {
        buf: data.to_owned().into(),
        counters: Default::default(),
    };

    let mut opcodes = vec![];

    while let Some(mut append) = Opcode::deserialize(&mut reader)? {
        opcodes.append(&mut append);
    }

    Ok(opcodes)
}
