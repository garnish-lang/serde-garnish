use std::convert::From;
use std::fmt::format;

use serde::de::Visitor;
use serde::Deserializer;

use garnish_traits::{ExpressionDataType, GarnishLangRuntimeData};

use crate::error::{wrap_err, GarnishSerializationError};
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

    fn value(&self) -> Result<(ExpressionDataType, Data::Size), GarnishSerializationError<Data>> {
        let a = self
            .data
            .get_current_value()
            .ok_or("No current value to deserialize.")?;
        let t = self
            .data
            .get_data_type(a)
            .or_else(|e| Err(GarnishSerializationError::new(e)))?;

        Ok((t, a))
    }

    fn deserialize_primitive<'de, From, To, V, GetF, VisitF>(
        &self,
        visitor: V,
        get_source: GetF,
        visit_func: VisitF,
        expected_type: ExpressionDataType,
    ) -> Result<V::Value, GarnishSerializationError<Data>>
    where
        V: Visitor<'de>,
        From: Into<To>,
        GetF: FnOnce(&Data, Data::Size) -> Result<From, Data::Error>,
        VisitF: FnOnce(V, To) -> Result<V::Value, GarnishSerializationError<Data>>,
    {
        let (t, a) = self.value()?;
        match t == expected_type {
            true => {
                let v = get_source(self.data, a).or_else(wrap_err)?;
                visit_func(visitor, v.into())
            }
            false => Err(GarnishSerializationError::from(
                format!("Expected {:?}, found {:?}", expected_type, t).as_str(),
            )),
        }
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
        let (t, _a) = self.value()?;

        match t {
            ExpressionDataType::True => visitor.visit_bool(true),
            ExpressionDataType::False => visitor.visit_bool(false),
            t => Err(GarnishSerializationError::from(
                format!("Expected True or False, found {:?}", t).as_str(),
            )),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_i8,
            ExpressionDataType::Number,
        )
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
    use std::fmt::Debug;
    use serde::de::DeserializeOwned;
    use serde::Deserialize;

    use garnish_data::data::SimpleNumber;
    use garnish_data::{DataError, SimpleRuntimeData};
    use garnish_traits::GarnishLangRuntimeData;

    use crate::deserializer::GarnishDataDeserializer;

    fn assert_deserializes<SetupF, Type>(setup: SetupF, expected_value: Type)
        where
            SetupF: FnOnce(&mut SimpleRuntimeData) -> Result<usize, DataError>,
            Type: DeserializeOwned + PartialEq + Eq + Debug,
    {
        let mut data = SimpleRuntimeData::new();
        let addr = setup(&mut data).unwrap();
        data.push_value_stack(addr).unwrap();

        let mut deserializer = GarnishDataDeserializer::new(&mut data);

        let v = Type::deserialize(&mut deserializer).unwrap();

        assert_eq!(v, expected_value)
    }

    #[test]
    fn deserialize_true() {
        assert_deserializes(|data| {
            data.add_true()
        }, true);
    }

    #[test]
    fn deserialize_false() {
        assert_deserializes(|data| {
            data.add_false()
        }, false);
    }

    #[test]
    fn deserialize_i8() {
        assert_deserializes(|data| {
            data.add_number(SimpleNumber::Integer(100))
        }, 100i8);
    }
}
