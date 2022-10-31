use std::fmt::format;

use serde::de::Visitor;
use serde::Deserializer;

use garnish_traits::{ExpressionDataType, GarnishLangRuntimeData};

use crate::error::GarnishSerializationError;
use crate::serializer::GarnishNumberConversions;

struct GarnishDataDeserializer<'a, Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    data: &'a Data,
}

impl<'a, Data> GarnishDataDeserializer<'a, Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    pub fn new(data: &'a Data) -> Self {
        Self { data }
    }
}

impl<'de, 'a, 'b, Data> Deserializer<'de> for &'b mut GarnishDataDeserializer<'a, Data>
where
    'a: 'de,
    'a: 'b,
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    type Error = GarnishSerializationError<Data>;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let t = self
            .data
            .get_data_type(
                self.data
                    .get_current_value()
                    .ok_or("No current value to deserialize.")?,
            )
            .or_else(|e| Err(GarnishSerializationError::new(e)))?;

        match t {
            ExpressionDataType::True => visitor.visit_bool(true),
            ExpressionDataType::False => visitor.visit_bool(false),
            t => Err(GarnishSerializationError::from(format!(
                "Expected True or False, found {:?}",
                t
            ).as_str())),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_unit_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_newtype_struct<V>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_tuple_struct<V>(
        self,
        name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_struct<V>(
        self,
        name: &'static str,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use serde::Deserialize;

    use garnish_data::SimpleRuntimeData;
    use garnish_traits::GarnishLangRuntimeData;

    use crate::deserializer::GarnishDataDeserializer;

    #[test]
    fn deserialize_true() {
        let mut data = SimpleRuntimeData::new();
        data.add_true()
            .and_then(|r| data.push_value_stack(r))
            .unwrap();

        let mut deserializer = GarnishDataDeserializer::new(&mut data);

        let v = bool::deserialize(&mut deserializer).unwrap();

        assert!(v);
    }

    #[test]
    fn deserialize_false() {
        let mut data = SimpleRuntimeData::new();
        data.add_false()
            .and_then(|r| data.push_value_stack(r))
            .unwrap();

        let mut deserializer = GarnishDataDeserializer::new(&mut data);

        let v = bool::deserialize(&mut deserializer).unwrap();

        assert!(!v);
    }
}
