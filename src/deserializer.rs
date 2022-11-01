use std::convert::From;
use std::fmt::format;

use serde::de::Visitor;
use serde::Deserializer;

use garnish_traits::{ExpressionDataType, GarnishLangRuntimeData, TypeConstants};

use crate::error::{wrap_err, GarnishSerializationError};
use crate::serializer::GarnishNumberConversions;

struct GarnishDataDeserializer<'a, Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Size: Into<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
    Data::Byte: Into<u8>,
{
    data: &'a Data,
}

impl<'a, Data> GarnishDataDeserializer<'a, Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Size: Into<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
    Data::Byte: Into<u8>,
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
    Data::Size: Into<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
    Data::Byte: Into<u8>,
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
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_i16,
            ExpressionDataType::Number,
        )
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_i32,
            ExpressionDataType::Number,
        )
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_i64,
            ExpressionDataType::Number,
        )
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_u8,
            ExpressionDataType::Number,
        )
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_u16,
            ExpressionDataType::Number,
        )
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_u32,
            ExpressionDataType::Number,
        )
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_u64,
            ExpressionDataType::Number,
        )
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_f32,
            ExpressionDataType::Number,
        )
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_f64,
            ExpressionDataType::Number,
        )
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_char,
            V::visit_char,
            ExpressionDataType::Char,
        )
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(GarnishSerializationError::from(
            "Deserialization of &str not supported, use owned type String instead.",
        ))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (t, a) = self.value()?;
        match t {
            ExpressionDataType::CharList => {
                let len = self.data.get_char_list_len(a).or_else(wrap_err)?;
                let mut s = String::with_capacity(len.into());
                let mut i = Data::Size::zero();

                while i < len {
                    let c = self.data.get_char_list_item(a, Data::size_to_number(i)).or_else(wrap_err)?;
                    s.push(c.into());
                    i += Data::Size::one();
                }

                visitor.visit_string(s)
            }
            t => Err(GarnishSerializationError::from(format!("Expected CharList, found {:?}", t).as_str()))
        }
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        Err(GarnishSerializationError::from(
            "Deserialization of &[u8] not supported, use owned type Vec<u8> instead.",
        ))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (t, a) = self.value()?;
        match t {
            ExpressionDataType::ByteList => {
                let len = self.data.get_byte_list_len(a).or_else(wrap_err)?;
                let mut bytes = Vec::with_capacity(len.into());
                let mut i = Data::Size::zero();

                while i < len {
                    let b = self.data.get_byte_list_item(a, Data::size_to_number(i)).or_else(wrap_err)?;
                    bytes.push(b.into());
                    i += Data::Size::one();
                }

                visitor.visit_byte_buf(bytes)
            }
            t => Err(GarnishSerializationError::from(format!("Expected ByteList, found {:?}", t).as_str()))
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (t, _a) = self.value()?;
        match t {
            ExpressionDataType::Unit => visitor.visit_none(),
            _ => visitor.visit_some(self)
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'de>,
    {
        let (t, _a) = self.value()?;
        match t {
            ExpressionDataType::Unit => visitor.visit_unit(),
            t => Err(GarnishSerializationError::from(format!("Expected Unit, found {:?}", t).as_str()))
        }
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
    use std::fmt::{Debug, Formatter};

    use serde::de::{DeserializeOwned, Error, Visitor};
    use serde::{Deserialize, Deserializer};

    use garnish_data::data::SimpleNumber;
    use garnish_data::{DataError, SimpleRuntimeData};
    use garnish_traits::GarnishLangRuntimeData;

    use crate::deserializer::GarnishDataDeserializer;

    fn assert_deserializes<SetupF, Type>(setup: SetupF, expected_value: Type)
    where
        SetupF: FnOnce(&mut SimpleRuntimeData) -> Result<usize, DataError>,
        Type: DeserializeOwned + PartialEq + Debug,
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
        assert_deserializes(|data| data.add_true(), true);
    }

    #[test]
    fn deserialize_false() {
        assert_deserializes(|data| data.add_false(), false);
    }

    #[test]
    fn deserialize_i8() {
        assert_deserializes(|data| data.add_number(SimpleNumber::Integer(100)), 100i8);
    }

    #[test]
    fn deserialize_i16() {
        assert_deserializes(|data| data.add_number(SimpleNumber::Integer(100)), 100i16);
    }

    #[test]
    fn deserialize_i32() {
        assert_deserializes(|data| data.add_number(SimpleNumber::Integer(100)), 100i32);
    }

    #[test]
    fn deserialize_i64() {
        assert_deserializes(|data| data.add_number(SimpleNumber::Integer(100)), 100i64);
    }

    #[test]
    fn deserialize_u8() {
        assert_deserializes(|data| data.add_number(SimpleNumber::Integer(100)), 100u8);
    }

    #[test]
    fn deserialize_u16() {
        assert_deserializes(|data| data.add_number(SimpleNumber::Integer(100)), 100u16);
    }

    #[test]
    fn deserialize_u32() {
        assert_deserializes(|data| data.add_number(SimpleNumber::Integer(100)), 100u32);
    }

    #[test]
    fn deserialize_u64() {
        assert_deserializes(|data| data.add_number(SimpleNumber::Integer(100)), 100u64);
    }

    #[test]
    fn deserialize_f32() {
        assert_deserializes(|data| data.add_number(SimpleNumber::Float(100.0)), 100.0f32);
    }

    #[test]
    fn deserialize_f64() {
        assert_deserializes(|data| data.add_number(SimpleNumber::Float(100.0)), 100.0f64);
    }

    #[test]
    fn deserialize_char() {
        assert_deserializes(|data| data.add_char('a'), 'a');
    }

    // cannot currently be implemented
    // #[test]
    // fn deserialize_str() {
    //     assert_deserializes(|data| {
    //         data.parse_add_char_list("abcd")
    //     }, "abcd");
    // }

    #[test]
    fn deserialize_string() {
        assert_deserializes(
            |data| data.parse_add_char_list("abcd"),
            String::from("abcd"),
        );
    }

    // cannot currently be implemented
    // #[test]
    // fn deserialize_bytes() {
    //     assert_deserializes(|data| {
    //         data.parse_add_byte_list("abcd")
    //     }, &['a' as u8, 'b' as u8, 'c' as u8, 'd' as u8]);
    // }

    // regular vec was calling sequence deserialization
    // made this one to ensure byte buf functions are called
    #[derive(Debug, Clone, PartialEq)]
    struct SomeBytes {
        bytes: Vec<u8>
    }

    impl<'de> Deserialize<'de> for SomeBytes {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
            struct SomeBytesVisitor;
            impl<'de> Visitor<'de> for SomeBytesVisitor {
                type Value = SomeBytes;

                fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                    formatter.write_str("Expecting vec of bytes.")
                }

                fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E> where E: Error {
                    Ok(SomeBytes { bytes: v })
                }
            }

            deserializer.deserialize_byte_buf(SomeBytesVisitor)
        }
    }

    #[test]
    fn deserialize_byte_buf() {
        assert_deserializes(|data| {
            data.parse_add_byte_list("abcd")
        }, SomeBytes { bytes: vec!['a' as u8, 'b' as u8, 'c' as u8, 'd' as u8] });
    }

    #[test]
    fn deserialize_option_some() {
        assert_deserializes(|data| {
            data.add_number(SimpleNumber::Integer(100))
        }, Some(100));
    }

    #[test]
    fn deserialize_option_none() {
        assert_deserializes(|data| {
            data.add_unit()
        }, None::<i32>);
    }

    #[test]
    fn deserialize_unit() {
        assert_deserializes(|data| {
            data.add_unit()
        }, ());
    }
}
