use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::{ser, Serialize, Serializer};

use garnish_traits::GarnishLangRuntimeData;

pub trait GarnishNumberConversions:
    From<i8>
    + From<i16>
    + From<i32>
    + From<i64>
    + From<u8>
    + From<u16>
    + From<u32>
    + From<u64>
    + From<f32>
    + From<f64>
{
}

impl<T> GarnishNumberConversions for T where
    T: From<i8>
        + From<i16>
        + From<i32>
        + From<i64>
        + From<u8>
        + From<u16>
        + From<u32>
        + From<u64>
        + From<f32>
        + From<f64>
{
}

struct GarnishSerializationError<Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    err: Data::Error,
}

impl<Data> GarnishSerializationError<Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    pub fn new(err: Data::Error) -> Self {
        Self { err }
    }
}

impl<Data> Debug for GarnishSerializationError<Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self.err).as_str())
    }
}

impl<Data> Display for GarnishSerializationError<Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.err).as_str())
    }
}

impl<Data> Error for GarnishSerializationError<Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
}

impl<Data> ser::Error for GarnishSerializationError<Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        todo!()
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum OptionalBehavior {
    Pair,
    UnitValue,
}

struct GarnishDataSerializer<'a, Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    data: &'a mut Data,
    data_addr: Option<Data::Size>,
    optional_behavior: OptionalBehavior,
}

impl<'a, Data> GarnishDataSerializer<'a, Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    pub fn new(data: &'a mut Data) -> Self {
        GarnishDataSerializer {
            data,
            data_addr: None,
            optional_behavior: OptionalBehavior::Pair
        }
    }

    pub fn data_addr(&self) -> Option<Data::Size> {
        self.data_addr
    }

    pub fn add_convertible_number<T>(
        &mut self,
        v: T,
    ) -> Result<Data::Size, GarnishSerializationError<Data>>
    where
        Data::Number: From<T>,
    {
        self.data
            .add_number(Data::Number::from(v))
            .or_else(wrap_err)
    }

    pub fn set_optional_behavior(&mut self, behavior: OptionalBehavior) {
        self.optional_behavior = behavior;
    }
}

fn wrap_err<V, Data>(e: Data::Error) -> Result<V, GarnishSerializationError<Data>>
where
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    Err(GarnishSerializationError::new(e))
}

impl<'a, 'b, Data> Serializer for &'b mut GarnishDataSerializer<'a, Data>
where
    'a: 'b,
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    type Ok = Data::Size;
    type Error = GarnishSerializationError<Data>;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        match v {
            true => self.data.add_true().or_else(wrap_err),
            false => self.data.add_false().or_else(wrap_err),
        }
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.add_convertible_number(v)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.add_convertible_number(v)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.add_convertible_number(v)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        self.add_convertible_number(v)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.add_convertible_number(v)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.add_convertible_number(v)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.add_convertible_number(v)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        self.add_convertible_number(v)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.add_convertible_number(v)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.add_convertible_number(v)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.data.add_char(Data::Char::from(v)).or_else(wrap_err)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        self.data.start_char_list().or_else(wrap_err)?;
        for c in v.chars() {
            self.data.add_to_char_list(Data::Char::from(c)).or_else(wrap_err)?;
        }

        self.data.end_char_list().or_else(wrap_err)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.data.start_byte_list().or_else(wrap_err)?;
        for b in v {
            self.data.add_to_byte_list(Data::Byte::from(*b)).or_else(wrap_err)?;
        }
        
        self.data.end_byte_list().or_else(wrap_err)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        let unit = self.data.add_unit().or_else(wrap_err)?;
        match self.optional_behavior {
            OptionalBehavior::Pair => {
                let sym = self.data.parse_add_symbol("none").or_else(wrap_err)?;
                self.data.add_pair((sym, unit)).or_else(wrap_err)
            }
            OptionalBehavior::UnitValue => {
                // already added to data, return addr
                Ok(unit)
            }
        }
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let val = value.serialize(&mut *self)?;
        match self.optional_behavior {
            OptionalBehavior::Pair => {
                let sym = self.data.parse_add_symbol("some").or_else(wrap_err)?;
                self.data.add_pair((sym, val)).or_else(wrap_err)
            }
            OptionalBehavior::UnitValue => {
                // already added to data, return addr
                Ok(val)
            }
        }
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

impl<'a, 'b, Data> SerializeSeq for &'b mut GarnishDataSerializer<'a, Data>
where
    'a: 'b,
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    type Ok = Data::Size;
    type Error = GarnishSerializationError<Data>;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, 'b, Data> SerializeMap for &'b mut GarnishDataSerializer<'a, Data>
where
    'a: 'b,
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    type Ok = Data::Size;
    type Error = GarnishSerializationError<Data>;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, 'b, Data> SerializeStruct for &'b mut GarnishDataSerializer<'a, Data>
where
    'a: 'b,
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    type Ok = Data::Size;
    type Error = GarnishSerializationError<Data>;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, 'b, Data> SerializeStructVariant for &'b mut GarnishDataSerializer<'a, Data>
where
    'a: 'b,
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    type Ok = Data::Size;
    type Error = GarnishSerializationError<Data>;

    fn serialize_field<T: ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, 'b, Data> SerializeTuple for &'b mut GarnishDataSerializer<'a, Data>
where
    'a: 'b,
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    type Ok = Data::Size;
    type Error = GarnishSerializationError<Data>;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, 'b, Data> SerializeTupleStruct for &'b mut GarnishDataSerializer<'a, Data>
where
    'a: 'b,
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    type Ok = Data::Size;
    type Error = GarnishSerializationError<Data>;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, 'b, Data> SerializeTupleVariant for &'b mut GarnishDataSerializer<'a, Data>
where
    'a: 'b,
    Data: GarnishLangRuntimeData,
    Data::Number: GarnishNumberConversions,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    type Ok = Data::Size;
    type Error = GarnishSerializationError<Data>;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use serde::Serializer;

    use garnish_data::data::{SimpleData, SimpleNumber};
    use garnish_data::{SimpleRuntimeData, symbol_value};

    use crate::serializer::{GarnishDataSerializer, OptionalBehavior};

    #[test]
    fn serialize_true() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_bool(true).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::True);
    }

    #[test]
    fn serialize_false() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_bool(false).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::False);
    }

    #[test]
    fn serialize_i8() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_i8(125).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Integer(125)));
    }

    #[test]
    fn serialize_i16() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_i16(125).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Integer(125)));
    }

    #[test]
    fn serialize_i32() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_i32(125).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Integer(125)));
    }

    #[test]
    fn serialize_i64() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_i64(125).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Integer(125)));
    }

    #[test]
    fn serialize_u8() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_u8(125).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Integer(125)));
    }

    #[test]
    fn serialize_u16() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_u16(125).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Integer(125)));
    }

    #[test]
    fn serialize_u32() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_u32(125).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Integer(125)));
    }

    #[test]
    fn serialize_u64() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_u64(125).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Integer(125)));
    }

    #[test]
    fn serialize_f32() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_f32(125.0).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Float(125.0)));
    }

    #[test]
    fn serialize_f64() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_f64(125.0).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Float(125.0)));
    }

    #[test]
    fn serialize_char() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_char('a').unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Char('a'));
    }

    #[test]
    fn serialize_str() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_str("abcd").unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::CharList("abcd".to_string()));
    }

    #[test]
    fn serialize_byte() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_bytes(&[1, 2, 3, 4]).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::ByteList(vec![1, 2, 3, 4]));
    }

    #[test]
    fn serialize_none_pair() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_none().unwrap();

        let (left, right) = data.get_data().get(addr).unwrap().as_pair().unwrap();
        assert_eq!(data.get_data().get(left).unwrap(), &SimpleData::Symbol(symbol_value("none")));
        assert_eq!(data.get_data().get(right).unwrap(), &SimpleData::Unit);
    }

    #[test]
    fn serialize_some_pair() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_some(&10).unwrap();

        let (left, right) = data.get_data().get(addr).unwrap().as_pair().unwrap();

        assert_eq!(data.get_data().get(left).unwrap(), &SimpleData::Symbol(symbol_value("some")));
        assert_eq!(data.get_data().get(right).unwrap(), &SimpleData::Number(SimpleNumber::Integer(10)));
    }

    #[test]
    fn serialize_none_as_unit() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);
        serializer.set_optional_behavior(OptionalBehavior::UnitValue);

        let addr = serializer.serialize_none().unwrap();

        assert_eq!(data.get_data().get(addr).unwrap(), &SimpleData::Unit);
    }

    #[test]
    fn serialize_some_as_value() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);
        serializer.set_optional_behavior(OptionalBehavior::UnitValue);

        let addr = serializer.serialize_some(&10).unwrap();

        assert_eq!(data.get_data().get(addr).unwrap(), &SimpleData::Number(SimpleNumber::Integer(10)));
    }
}
