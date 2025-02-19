use mlang_rs::rt::serde::de::{Deserialize, Deserializer, NodeAccess};

/// Variant deserializer.
pub(super) struct Variant<'a>(pub(super) &'a str);

#[allow(unused)]
impl<'a> Deserializer for &'a mut Variant<'a> {
    type Error = mlang_rs::rt::serde::de::Error;

    fn deserialize_opcode<V>(self, visitor: V) -> Result<Option<Vec<V::Value>>, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_element<V>(
        self,
        type_id: usize,
        name: &str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_leaf<V>(
        self,
        type_id: usize,
        name: &str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_attr<V>(
        self,
        type_id: usize,
        name: &str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_data<V>(
        self,
        type_id: usize,
        name: &str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        type_id: usize,
        name: &str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        visitor.visit_enum_with(self.0, self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<Option<V::Value>, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_variable<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_byte<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_ubyte<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_short<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_ushort<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_int<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_uint<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_long<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_ulong<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_float<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }

    fn deserialize_double<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: mlang_rs::rt::serde::de::Visitor,
    {
        todo!()
    }
}

#[allow(unused)]
impl<'a> NodeAccess for &'a mut Variant<'a> {
    type Error = mlang_rs::rt::serde::de::Error;

    fn deserialize_field<T>(
        &mut self,
        ty: &str,
        index: usize,
        field_name: Option<&str>,
    ) -> Result<T::Value, Self::Error>
    where
        T: Deserialize,
    {
        todo!()
    }
}
