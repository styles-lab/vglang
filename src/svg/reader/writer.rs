//! A serializer to create reading [`code`](super::state::ReadingCode) from vglang data type.
//!

use mlang_rs::rt::serde::ser::{SerializeNode, SerializeSeq, Serializer};

use super::{state::ReadingCode, ReadingError};

#[derive(Default)]
pub struct ReadingCodeWriter(pub(super) Vec<ReadingCode>);

impl<'a> SerializeNode for &'a mut ReadingCodeWriter {
    type Error = ReadingError;

    fn serialize_field<T>(
        &mut self,
        index: usize,
        name: Option<&str>,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: ?Sized + mlang_rs::rt::serde::ser::Serialize,
    {
        self.0.push(ReadingCode::Field {
            name: name.map(|v| v.to_string()),
            index,
        });

        value.serialize(&mut **self)
    }

    fn finish(self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<'a> SerializeSeq for &'a mut ReadingCodeWriter {
    type Error = ReadingError;

    fn next_item<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + mlang_rs::rt::serde::ser::Serialize,
    {
        value.serialize(&mut **self)
    }

    fn finish(self) -> Result<(), Self::Error> {
        self.0.push(ReadingCode::SeqEnd);
        Ok(())
    }
}

impl<'a> Serializer for &'a mut ReadingCodeWriter {
    type Error = ReadingError;

    type SerializeNode = Self;

    type SerializeSeq = Self;

    fn serialize_el(
        self,
        _: usize,
        name: &str,
        _: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        self.0.push(ReadingCode::Elem(name.to_string()));
        Ok(self)
    }

    fn serialize_leaf(
        self,
        _: usize,
        name: &str,
        _: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        self.0.push(ReadingCode::Leaf(name.to_string()));
        Ok(self)
    }

    fn serialize_attr(
        self,
        _: usize,
        name: &str,
        _: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        self.0.push(ReadingCode::Attr(name.to_string()));
        Ok(self)
    }

    fn serialize_data(
        self,
        _: usize,
        name: &str,
        _: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        self.0.push(ReadingCode::Data(name.to_string()));
        Ok(self)
    }

    fn serialize_enum(
        self,
        _: usize,
        _: &str,
        variant: &str,
        _: usize,
        _: usize,
    ) -> Result<Self::SerializeNode, Self::Error> {
        self.0.push(ReadingCode::Variant(variant.to_string()));
        Ok(self)
    }

    fn serialize_seq(self, _: usize) -> Result<Self::SerializeSeq, Self::Error> {
        self.0.push(ReadingCode::SeqStart);
        Ok(self)
    }

    fn serialize_bool(self, value: bool) -> Result<(), Self::Error> {
        self.serialize_string(if value { "1" } else { "0" })
    }

    fn serialize_string(self, value: &str) -> Result<(), Self::Error> {
        self.0.push(ReadingCode::Value(value.to_string()));

        Ok(())
    }

    fn serialize_byte(self, value: i8) -> Result<(), Self::Error> {
        self.0.push(ReadingCode::Value(format!("{}", value)));

        Ok(())
    }

    fn serialize_ubyte(self, value: u8) -> Result<(), Self::Error> {
        self.0.push(ReadingCode::Value(format!("{}", value)));

        Ok(())
    }

    fn serialize_short(self, value: i16) -> Result<(), Self::Error> {
        self.0.push(ReadingCode::Value(format!("{}", value)));

        Ok(())
    }

    fn serialize_ushort(self, value: u16) -> Result<(), Self::Error> {
        self.0.push(ReadingCode::Value(format!("{}", value)));

        Ok(())
    }

    fn serialize_int(self, value: i32) -> Result<(), Self::Error> {
        self.0.push(ReadingCode::Value(format!("{}", value)));

        Ok(())
    }

    fn serialize_uint(self, value: u32) -> Result<(), Self::Error> {
        self.0.push(ReadingCode::Value(format!("{}", value)));

        Ok(())
    }

    fn serialize_long(self, value: i64) -> Result<(), Self::Error> {
        self.0.push(ReadingCode::Value(format!("{}", value)));

        Ok(())
    }

    fn serialize_ulong(self, value: u64) -> Result<(), Self::Error> {
        self.0.push(ReadingCode::Value(format!("{}", value)));

        Ok(())
    }

    fn serialize_float(self, value: f32) -> Result<(), Self::Error> {
        self.0.push(ReadingCode::Value(format!("{}", value)));

        Ok(())
    }

    fn serialize_double(self, value: f64) -> Result<(), Self::Error> {
        self.0.push(ReadingCode::Value(format!("{}", value)));

        Ok(())
    }

    fn serialize_none(self) -> Result<(), Self::Error> {
        self.0.push(ReadingCode::None);

        Ok(())
    }

    fn serialize_variable(
        self,
        _: &mlang_rs::rt::opcode::Path,
        _: &mlang_rs::rt::opcode::Target,
    ) -> Result<(), Self::Error> {
        panic!("unsupport: serialize_variable")
    }

    fn serialize_pop(self) -> Result<(), Self::Error> {
        panic!("unsupport: serialize_pop")
    }
}

#[cfg(test)]
mod tests {
    use mlang_rs::rt::serde::ser::Serialize;

    use crate::opcode::{Color, ColorKeyWord};

    use super::*;

    #[test]
    fn test_writer() {
        let mut writer = ReadingCodeWriter::default();

        true.serialize(&mut writer).expect("true");
        false.serialize(&mut writer).expect("false");

        ColorKeyWord::Aliceblue
            .serialize(&mut writer)
            .expect("ColorKeyWord::Aliceblue");

        Color::Rgb(1, 1, 1)
            .serialize(&mut writer)
            .expect("rgb(1,1,1)");

        assert_eq!(
            writer.0,
            vec![
                ReadingCode::Value("1".to_string()),
                ReadingCode::Value("0".to_string()),
                ReadingCode::Variant("aliceblue".to_string()),
                ReadingCode::Variant("rgb".to_string()),
                ReadingCode::Field {
                    name: None,
                    index: 0
                },
                ReadingCode::Value("1".to_string()),
                ReadingCode::Field {
                    name: None,
                    index: 1
                },
                ReadingCode::Value("1".to_string()),
                ReadingCode::Field {
                    name: None,
                    index: 2
                },
                ReadingCode::Value("1".to_string()),
            ]
        )
    }
}
