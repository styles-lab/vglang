use bytes::{BufMut, BytesMut};
use mlang_rs::rt::serde::ser::{Serialize, SerializeNode, SerializeSeq, Serializer};

use crate::{binary::keyword::Keyword, opcode::Opcode};

/// Error type returns by this module.
#[derive(Debug, thiserror::Error)]
pub enum WriteError {
    #[error("Write data({0}) out of range: {1}")]
    OutOfRange(WriteKind, usize),
}

/// Error type returns by this module.
#[derive(Debug, thiserror::Error)]
pub enum WriteKind {
    #[error("string")]
    String,
    #[error("array/vec")]
    Seq,
}

const BINARY_WRITE_REPORT: &str = "BINARY_WRITE_REPORT";

#[derive(Default)]
struct BinaryWriter(BytesMut);

impl<'a> Serializer for &'a mut BinaryWriter {
    type Error = WriteError;

    type SerializeNode = Self;

    type SerializeSeq = Self;

    fn serialize_el(
        self,
        type_id: usize,
        name: &str,
        fields: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        assert!(
            fields < u8::MAX as usize,
            "el({})'s fields({}) is out of range.",
            name,
            fields
        );

        log::trace!(target: BINARY_WRITE_REPORT, "write el({})",name);

        let type_id = Keyword::to_binary_type_id(type_id);
        self.0.put_u8(type_id);
        // self.0.put_u8(fields as u8);

        Ok(self)
    }

    fn serialize_leaf(
        self,
        type_id: usize,
        name: &str,
        fields: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        assert!(
            fields < u8::MAX as usize,
            "leaf({})'s fields({}) is out of range.",
            name,
            fields
        );

        log::trace!(target: BINARY_WRITE_REPORT, "write leaf({})",name);

        let type_id = Keyword::to_binary_type_id(type_id);
        self.0.put_u8(type_id);
        // self.0.put_u8(fields as u8);

        Ok(self)
    }

    fn serialize_attr(
        self,
        type_id: usize,
        name: &str,
        fields: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        assert!(
            fields < u8::MAX as usize,
            "attr({})'s fields({}) is out of range.",
            name,
            fields
        );

        log::trace!(target: BINARY_WRITE_REPORT, "write attr({},{})",name,type_id);

        let type_id = Keyword::to_binary_type_id(type_id);
        self.0.put_u8(type_id);
        // self.0.put_u8(fields as u8);

        Ok(self)
    }

    fn serialize_data(
        self,
        type_id: usize,
        name: &str,
        fields: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        assert!(
            fields < u8::MAX as usize,
            "data({})'s fields({}) is out of range.",
            name,
            fields
        );

        log::trace!(target: BINARY_WRITE_REPORT, "write data({},{})",name,type_id);

        // let type_id = Keyword::to_binary_type_id(type_id);
        // self.0.put_u8(type_id);
        // self.0.put_u8(fields as u8);

        Ok(self)
    }

    fn serialize_enum(
        self,
        _: usize,
        name: &str,
        variant: &str,
        variant_index: usize,
        fields: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        assert!(
            fields < u8::MAX as usize,
            "enum({}::{})'s variant_index({}) is out of range.",
            name,
            variant,
            variant_index
        );

        assert!(
            fields < u8::MAX as usize,
            "enum({}::{})'s fields({}) is out of range.",
            name,
            variant,
            fields
        );

        log::trace!(target: BINARY_WRITE_REPORT, "write enum({}::{})",name,variant);

        // let type_id = Keyword::to_binary_type_id(type_id);

        // self.0.put_u8(type_id);
        self.0.put_u8(variant_index as u8);
        // self.0.put_u8(fields as u8);

        Ok(self)
    }

    fn serialize_seq(self, len: usize) -> Result<Self::SerializeSeq, Self::Error> {
        log::trace!(target: BINARY_WRITE_REPORT, "write seq({})",len);
        if len > u16::MAX as usize {
            return Err(WriteError::OutOfRange(WriteKind::String, len));
        }

        self.0.put_u8(Keyword::Seq.into());
        self.0.put_u16(len as u16);

        Ok(self)
    }

    fn serialize_bool(self, value: bool) -> Result<(), Self::Error> {
        log::trace!(target: BINARY_WRITE_REPORT, "write bool({})",value);
        self.0.put_u8(Keyword::Bool.into());
        self.0.put_u8(if value { 1 } else { 0 });

        Ok(())
    }

    fn serialize_string(self, value: &str) -> Result<(), Self::Error> {
        if value.len() > u16::MAX as usize {
            return Err(WriteError::OutOfRange(WriteKind::String, value.len()));
        }

        log::trace!(target: BINARY_WRITE_REPORT, "write string({})",value);

        self.0.put_u8(Keyword::String.into());
        self.0.put_u16(value.len() as u16);
        self.0.put(value.as_bytes());

        Ok(())
    }

    fn serialize_byte(self, value: i8) -> Result<(), Self::Error> {
        log::trace!(target: BINARY_WRITE_REPORT, "write byte({})",value);
        self.0.put_u8(Keyword::Byte.into());
        self.0.put_i8(value);

        Ok(())
    }

    fn serialize_ubyte(self, value: u8) -> Result<(), Self::Error> {
        log::trace!(target: BINARY_WRITE_REPORT, "write ubyte({})",value);
        self.0.put_u8(Keyword::Ubyte.into());
        self.0.put_u8(value);

        Ok(())
    }

    fn serialize_short(self, value: i16) -> Result<(), Self::Error> {
        log::trace!(target: BINARY_WRITE_REPORT, "write short({})",value);
        self.0.put_u8(Keyword::Short.into());
        self.0.put_i16(value);

        Ok(())
    }

    fn serialize_ushort(self, value: u16) -> Result<(), Self::Error> {
        log::trace!(target: BINARY_WRITE_REPORT, "write ushort({})",value);
        self.0.put_u8(Keyword::Ushort.into());
        self.0.put_u16(value);

        Ok(())
    }

    fn serialize_int(self, value: i32) -> Result<(), Self::Error> {
        log::trace!(target: BINARY_WRITE_REPORT, "write int({})",value);
        self.0.put_u8(Keyword::Int.into());
        self.0.put_i32(value);

        Ok(())
    }

    fn serialize_uint(self, value: u32) -> Result<(), Self::Error> {
        log::trace!(target: BINARY_WRITE_REPORT, "write uint({})",value);
        self.0.put_u8(Keyword::Uint.into());
        self.0.put_u32(value);

        Ok(())
    }

    fn serialize_long(self, value: i64) -> Result<(), Self::Error> {
        log::trace!(target: BINARY_WRITE_REPORT, "write long({})",value);
        self.0.put_u8(Keyword::Long.into());
        self.0.put_i64(value);

        Ok(())
    }

    fn serialize_ulong(self, value: u64) -> Result<(), Self::Error> {
        log::trace!(target: BINARY_WRITE_REPORT, "write ulong({})",value);
        self.0.put_u8(Keyword::Ulong.into());
        self.0.put_u64(value);

        Ok(())
    }

    fn serialize_float(self, value: f32) -> Result<(), Self::Error> {
        log::trace!(target: BINARY_WRITE_REPORT, "write float({})",value);
        self.0.put_u8(Keyword::Float.into());
        self.0.put_f32(value);

        Ok(())
    }

    fn serialize_double(self, value: f64) -> Result<(), Self::Error> {
        log::trace!(target: BINARY_WRITE_REPORT, "write double({})",value);
        self.0.put_u8(Keyword::Double.into());
        self.0.put_f64(value);

        Ok(())
    }

    fn serialize_none(self) -> Result<(), Self::Error> {
        log::trace!(target: BINARY_WRITE_REPORT, "write none");
        self.0.put_u8(Keyword::None.into());
        Ok(())
    }

    fn serialize_variable(
        self,
        path: &mlang_rs::rt::opcode::Path,
        target: &mlang_rs::rt::opcode::Target,
    ) -> Result<(), Self::Error> {
        log::trace!(target: BINARY_WRITE_REPORT, "write variable({:?},{:?})",path,target);
        self.0.put_u8(Keyword::Path.into());

        match path {
            mlang_rs::rt::opcode::Path::Named(v) => self.serialize_string(v)?,
            mlang_rs::rt::opcode::Path::Index(v) => self.serialize_ushort(*v as u16)?,
        }

        self.0.put_u8(Keyword::Target.into());

        match target {
            mlang_rs::rt::opcode::Target::Register => self.0.put_u8(0),
            mlang_rs::rt::opcode::Target::ForeachItem => self.0.put_u8(1),
            mlang_rs::rt::opcode::Target::ForeachIndex => self.0.put_u8(2),
            mlang_rs::rt::opcode::Target::Range => self.0.put_u8(3),
        }

        Ok(())
    }

    fn serialize_pop(self) -> Result<(), Self::Error> {
        log::trace!(target: BINARY_WRITE_REPORT, "write pop");
        self.0.put_u8(Keyword::Pop.into());

        Ok(())
    }
}

impl<'a> SerializeNode for &'a mut BinaryWriter {
    type Error = WriteError;

    fn serialize_field<T>(
        &mut self,
        index: usize,
        name: Option<&str>,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ?Sized + mlang_rs::rt::serde::ser::Serialize,
    {
        if let Some(name) = name {
            log::trace!(target: BINARY_WRITE_REPORT, "write filed({})",name);
        } else {
            log::trace!(target: BINARY_WRITE_REPORT, "write field({})",index);
        }

        value.serialize(&mut **self)
    }

    fn finish(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeSeq for &'a mut BinaryWriter {
    type Error = WriteError;

    fn next_item<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + mlang_rs::rt::serde::ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn finish(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// Decode opcodes from binary format.
pub fn to_binary(opcodes: impl AsRef<[Opcode]>) -> Result<Vec<u8>, WriteError> {
    let mut writer = BinaryWriter::default();

    for opcode in opcodes.as_ref() {
        opcode.serialize(&mut writer)?;
    }

    Ok(writer.0.into())
}
