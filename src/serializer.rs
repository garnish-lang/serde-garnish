use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::{ser, Serialize, Serializer};

use garnish_traits::{GarnishLangRuntimeData, TypeConstants};

struct GarnishDataSerializer<'a, Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: From<i8>,
{
    data: &'a mut Data,
    data_addr: Option<Data::Size>,
}

impl<'a, Data> GarnishDataSerializer<'a, Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: From<i8>,
{
    pub fn new(data: &'a mut Data) -> Self {
        GarnishDataSerializer {
            data,
            data_addr: None,
        }
    }

    pub fn data_addr(&self) -> Option<Data::Size> {
        self.data_addr
    }
}

struct GarnishSerializationError<Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: From<i8>,
{
    err: Data::Error,
}

impl<Data> GarnishSerializationError<Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: From<i8>,
{
    pub fn new(err: Data::Error) -> Self {
        Self { err }
    }
}

impl<Data> Debug for GarnishSerializationError<Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: From<i8>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self.err).as_str())
    }
}

impl<Data> Display for GarnishSerializationError<Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: From<i8>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.err).as_str())
    }
}

impl<Data> Error for GarnishSerializationError<Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: From<i8>,
{
}

impl<Data> ser::Error for GarnishSerializationError<Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: From<i8>,
{
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        todo!()
    }
}

impl<'a, Data> Serializer for &'a mut GarnishDataSerializer<'a, Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: From<i8>,
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
        todo!()
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.data
            .add_number(Data::Number::from(v))
            .or_else(|e| Err(GarnishSerializationError::new(e)))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        todo!()
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

impl<'a, Data> SerializeSeq for &'a mut GarnishDataSerializer<'a, Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: From<i8>,
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

impl<'a, Data> SerializeMap for &'a mut GarnishDataSerializer<'a, Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: From<i8>,
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

impl<'a, Data> SerializeStruct for &'a mut GarnishDataSerializer<'a, Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: From<i8>,
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

impl<'a, Data> SerializeStructVariant for &'a mut GarnishDataSerializer<'a, Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: From<i8>,
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

impl<'a, Data> SerializeTuple for &'a mut GarnishDataSerializer<'a, Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: From<i8>,
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

impl<'a, Data> SerializeTupleStruct for &'a mut GarnishDataSerializer<'a, Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: From<i8>,
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

impl<'a, Data> SerializeTupleVariant for &'a mut GarnishDataSerializer<'a, Data>
where
    Data: GarnishLangRuntimeData,
    Data::Number: From<i8>,
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
    use garnish_data::SimpleRuntimeData;

    use crate::serializer::GarnishDataSerializer;

    #[test]
    fn serialize_i8() {
        let mut data = SimpleRuntimeData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_i8(125).unwrap();

        let num = data.get_data().get(addr).unwrap();

        assert_eq!(num, &SimpleData::Number(SimpleNumber::Integer(125)));
    }
}
