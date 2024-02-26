use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant,
};
use serde::{Serialize, Serializer};

use garnish_lang_traits::{GarnishData, TypeConstants};

use crate::error::{wrap_err, GarnishSerializationError};
use crate::{
    GarnishNumberConversions, GarnishSerializationOptions, OptionalBehavior, StructBehavior,
    VariantNameBehavior,
};

pub struct GarnishDataSerializer<'a, Data>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    data: &'a mut Data,
    data_addr: Option<Data::Size>,
    struct_sym: Option<Data::Size>,
    pending_key: Option<Data::Size>,
    options: GarnishSerializationOptions,
}

impl<'a, Data> GarnishDataSerializer<'a, Data>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Char: From<char>,
    Data::Byte: From<u8>,
{
    pub fn new(data: &'a mut Data) -> Self {
        GarnishDataSerializer {
            data,
            data_addr: None,
            options: GarnishSerializationOptions::new(),
            struct_sym: None,
            pending_key: None,
        }
    }

    pub fn new_with_options(data: &'a mut Data, options: GarnishSerializationOptions) -> Self {
        Self {
            data,
            data_addr: None,
            struct_sym: None,
            pending_key: None,
            options,
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

    fn end_struct_like(&mut self) -> Result<Data::Size, GarnishSerializationError<Data>> {
        let list_addr = self.data.end_list().or_else(wrap_err)?;

        match self.struct_sym {
            Some(addr) => {
                self.data
                    .start_list(Data::Size::from(2))
                    .or_else(wrap_err)?;

                self.data.add_to_list(addr, false).or_else(wrap_err)?;
                self.data.add_to_list(list_addr, false).or_else(wrap_err)?;

                self.data.end_list().or_else(wrap_err)
            }
            // excluding name
            None => Ok(list_addr),
        }
    }
}

impl<'a, 'b, Data> Serializer for &'b mut GarnishDataSerializer<'a, Data>
where
    'a: 'b,
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
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
            self.data
                .add_to_char_list(Data::Char::from(c))
                .or_else(wrap_err)?;
        }

        self.data.end_char_list().or_else(wrap_err)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        self.data.start_byte_list().or_else(wrap_err)?;
        for b in v {
            self.data
                .add_to_byte_list(Data::Byte::from(*b))
                .or_else(wrap_err)?;
        }

        self.data.end_byte_list().or_else(wrap_err)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        match self.options.optional_behavior {
            OptionalBehavior::UnitSymbol => self.data.parse_add_symbol("none").or_else(wrap_err),
            OptionalBehavior::UnitValue => self.data.add_unit().or_else(wrap_err),
        }
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        self.data.add_unit().or_else(wrap_err)
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        match self.options.struct_typing_behavior {
            StructBehavior::ExcludeTyping => self.data.add_unit().or_else(wrap_err),
            StructBehavior::IncludeTyping => {
                let name_addr = self.data.parse_add_symbol(name).or_else(wrap_err)?;
                let v = self.data.add_unit().or_else(wrap_err)?;

                self.data.start_list(Data::Size::one()).or_else(wrap_err)?;
                self.data.add_to_list(name_addr, false).or_else(wrap_err)?;
                self.data.add_to_list(v, false).or_else(wrap_err)?;
                self.data.end_list().or_else(wrap_err)
            }
        }
    }

    fn serialize_unit_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        match self.options.variant_name_behavior {
            VariantNameBehavior::Short => self.data.parse_add_symbol(variant).or_else(wrap_err),
            VariantNameBehavior::Full => self
                .data
                .parse_add_symbol(format!("{}::{}", name, variant).as_str())
                .or_else(wrap_err),
            VariantNameBehavior::Index => self
                .data
                .add_number(Data::Number::from(variant_index))
                .or_else(wrap_err),
        }
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(&mut *self)
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
        self.data.start_list(Data::Size::one()).or_else(wrap_err)?;

        let sym = self.serialize_unit_variant(name, variant_index, variant)?;
        let value = value.serialize(&mut *self)?;

        self.data.add_to_list(sym, false).or_else(wrap_err)?;
        self.data.add_to_list(value, false).or_else(wrap_err)?;

        self.data.end_list().or_else(wrap_err)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        self.data
            .start_list(Data::Size::from(len.unwrap_or(0)))
            .or_else(wrap_err)?;
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        match self.options.struct_typing_behavior {
            StructBehavior::IncludeTyping => {
                self.struct_sym = Some(self.data.parse_add_symbol(name).or_else(wrap_err)?)
            }
            StructBehavior::ExcludeTyping => (),
        }
        self.serialize_seq(Some(len))
    }

    fn serialize_tuple_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        self.struct_sym = Some(self.serialize_unit_variant(name, variant_index, variant)?);
        self.serialize_seq(Some(len))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        self.serialize_seq(len)
    }

    fn serialize_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        match self.options.struct_typing_behavior {
            StructBehavior::IncludeTyping => {
                self.struct_sym = Some(self.data.parse_add_symbol(name).or_else(wrap_err)?)
            }
            StructBehavior::ExcludeTyping => (),
        }
        self.serialize_seq(Some(len))
    }

    fn serialize_struct_variant(
        self,
        name: &'static str,
        variant_index: u32,
        variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        self.struct_sym = Some(self.serialize_unit_variant(name, variant_index, variant)?);
        self.serialize_seq(Some(len))
    }
}

impl<'a, 'b, Data> SerializeSeq for &'b mut GarnishDataSerializer<'a, Data>
where
    'a: 'b,
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
{
    type Ok = Data::Size;
    type Error = GarnishSerializationError<Data>;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let addr = value.serialize(&mut **self)?;
        self.data.add_to_list(addr, false).or_else(wrap_err)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.data.end_list().or_else(wrap_err)
    }
}

impl<'a, 'b, Data> SerializeMap for &'b mut GarnishDataSerializer<'a, Data>
where
    'a: 'b,
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
{
    type Ok = Data::Size;
    type Error = GarnishSerializationError<Data>;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let v = key.serialize(&mut **self)?;
        // might not be a char list
        // let data perform conversion
        self.pending_key = Some(self.data.add_symbol_from(v).or_else(wrap_err)?);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        match self.pending_key {
            None => Err(GarnishSerializationError::from(
                "No key when serializing value for map.",
            )),
            Some(key) => {
                let val = value.serialize(&mut **self)?;
                let pair = self.data.add_pair((key, val)).or_else(wrap_err)?;
                self.data.add_to_list(pair, true).or_else(wrap_err)
            }
        }
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.data.end_list().or_else(wrap_err)
    }
}

impl<'a, 'b, Data> SerializeStruct for &'b mut GarnishDataSerializer<'a, Data>
where
    'a: 'b,
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
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
        let k = self.data.parse_add_symbol(key).or_else(wrap_err)?;
        let v = value.serialize(&mut **self)?;
        let p = self.data.add_pair((k, v)).or_else(wrap_err)?;
        self.data.add_to_list(p, true).or_else(wrap_err)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end_struct_like()
    }
}

impl<'a, 'b, Data> SerializeStructVariant for &'b mut GarnishDataSerializer<'a, Data>
where
    'a: 'b,
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
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
        let k = self.data.parse_add_symbol(key).or_else(wrap_err)?;
        let v = value.serialize(&mut **self)?;
        let p = self.data.add_pair((k, v)).or_else(wrap_err)?;
        self.data.add_to_list(p, true).or_else(wrap_err)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end_struct_like()
    }
}

impl<'a, 'b, Data> SerializeTuple for &'b mut GarnishDataSerializer<'a, Data>
where
    'a: 'b,
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
{
    type Ok = Data::Size;
    type Error = GarnishSerializationError<Data>;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let addr = value.serialize(&mut **self)?;
        self.data.add_to_list(addr, false).or_else(wrap_err)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.data.end_list().or_else(wrap_err)
    }
}

impl<'a, 'b, Data> SerializeTupleStruct for &'b mut GarnishDataSerializer<'a, Data>
where
    'a: 'b,
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
{
    type Ok = Data::Size;
    type Error = GarnishSerializationError<Data>;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let addr = value.serialize(&mut **self)?;
        self.data.add_to_list(addr, false).or_else(wrap_err)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end_struct_like()
    }
}

impl<'a, 'b, Data> SerializeTupleVariant for &'b mut GarnishDataSerializer<'a, Data>
where
    'a: 'b,
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
{
    type Ok = Data::Size;
    type Error = GarnishSerializationError<Data>;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let addr = value.serialize(&mut **self)?;
        self.data.add_to_list(addr, false).or_else(wrap_err)
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        self.end_struct_like()
    }
}

#[cfg(test)]
mod tests {
    use serde::Serializer;

    use garnish_lang_simple_data::data::{SimpleData, SimpleNumber};
    use garnish_lang_simple_data::{symbol_value, SimpleGarnishData};

    use crate::serializer::{
        GarnishDataSerializer, OptionalBehavior, StructBehavior, VariantNameBehavior,
    };
    use crate::GarnishSerializationOptions;

    #[test]
    fn serialize_true() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_bool(true).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::True);
    }

    #[test]
    fn serialize_false() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_bool(false).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::False);
    }

    #[test]
    fn serialize_i8() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_i8(125).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Integer(125)));
    }

    #[test]
    fn serialize_i16() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_i16(125).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Integer(125)));
    }

    #[test]
    fn serialize_i32() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_i32(125).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Integer(125)));
    }

    #[test]
    fn serialize_i64() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_i64(125).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Integer(125)));
    }

    #[test]
    fn serialize_u8() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_u8(125).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Integer(125)));
    }

    #[test]
    fn serialize_u16() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_u16(125).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Integer(125)));
    }

    #[test]
    fn serialize_u32() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_u32(125).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Integer(125)));
    }

    #[test]
    fn serialize_u64() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_u64(125).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Integer(125)));
    }

    #[test]
    fn serialize_f32() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_f32(125.0).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Float(125.0)));
    }

    #[test]
    fn serialize_f64() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_f64(125.0).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Number(SimpleNumber::Float(125.0)));
    }

    #[test]
    fn serialize_char() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_char('a').unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::Char('a'));
    }

    #[test]
    fn serialize_str() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_str("abcd").unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::CharList("abcd".to_string()));
    }

    #[test]
    fn serialize_byte() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_bytes(&[1, 2, 3, 4]).unwrap();

        let num = data.get_data().get(addr).unwrap();
        assert_eq!(num, &SimpleData::ByteList(vec![1, 2, 3, 4]));
    }

    #[test]
    fn serialize_none_symbol() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new_with_options(
            &mut data,
            GarnishSerializationOptions::new().optional_behavior(OptionalBehavior::UnitSymbol),
        );

        let addr = serializer.serialize_none().unwrap();

        assert_eq!(
            data.get_data().get(addr).unwrap(),
            &SimpleData::Symbol(symbol_value("none"))
        );
    }

    #[test]
    fn serialize_some_as_value_when_unit_symbol() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new_with_options(
            &mut data,
            GarnishSerializationOptions::new().optional_behavior(OptionalBehavior::UnitSymbol),
        );

        let addr = serializer.serialize_some(&10).unwrap();

        assert_eq!(
            data.get_data().get(addr).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(10))
        );
    }

    #[test]
    fn serialize_none_as_unit() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_none().unwrap();

        assert_eq!(data.get_data().get(addr).unwrap(), &SimpleData::Unit);
    }

    #[test]
    fn serialize_some_as_value() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_some(&10).unwrap();

        assert_eq!(
            data.get_data().get(addr).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(10))
        );
    }

    #[test]
    fn serialize_unit() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_unit().unwrap();

        assert_eq!(data.get_data().get(addr).unwrap(), &SimpleData::Unit);
    }

    #[test]
    fn serialize_unit_struct_as_unit() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer.serialize_unit_struct("PhantomData").unwrap();

        assert_eq!(data.get_data().get(addr).unwrap(), &SimpleData::Unit);
    }

    #[test]
    fn serialize_unit_struct_as_list() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new_with_options(
            &mut data,
            GarnishSerializationOptions::new().struct_typing_behavior(StructBehavior::IncludeTyping),
        );

        let addr = serializer.serialize_unit_struct("PhantomData").unwrap();

        let list = data.get_data().get(addr).unwrap().as_list().unwrap().0;

        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Symbol(symbol_value("PhantomData"))
        );
        assert_eq!(
            data.get_data().get(*list.get(1).unwrap()).unwrap(),
            &SimpleData::Unit
        );
    }

    #[test]
    fn serialize_variant_full_name() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer
            .serialize_unit_variant("MyEnum", 100, "Value1")
            .unwrap();

        assert_eq!(
            data.get_data().get(addr).unwrap(),
            &SimpleData::Symbol(symbol_value("MyEnum::Value1"))
        );
    }

    #[test]
    fn serialize_variant_short_name() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new_with_options(
            &mut data,
            GarnishSerializationOptions::new().variant_name_behavior(VariantNameBehavior::Short),
        );

        let addr = serializer
            .serialize_unit_variant("MyEnum", 100, "Value1")
            .unwrap();

        assert_eq!(
            data.get_data().get(addr).unwrap(),
            &SimpleData::Symbol(symbol_value("Value1"))
        );
    }

    #[test]
    fn serialize_variant_index() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new_with_options(
            &mut data,
            GarnishSerializationOptions::new().variant_name_behavior(VariantNameBehavior::Index),
        );

        let addr = serializer
            .serialize_unit_variant("MyEnum", 100, "Value1")
            .unwrap();

        assert_eq!(
            data.get_data().get(addr).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(100))
        );
    }

    #[test]
    fn serialize_new_type_variant_full_name() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let addr = serializer
            .serialize_newtype_variant("MyEnum", 100, "Value1", &200)
            .unwrap();

        let list = data.get_data().get(addr).unwrap().as_list().unwrap().0;

        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Symbol(symbol_value("MyEnum::Value1"))
        );
        assert_eq!(
            data.get_data().get(*list.get(1).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(200))
        );
    }

    #[test]
    fn serialize_new_type_variant_short_name() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new_with_options(
            &mut data,
            GarnishSerializationOptions::new().variant_name_behavior(VariantNameBehavior::Short),
        );

        let addr = serializer
            .serialize_newtype_variant("MyEnum", 100, "Value1", &200)
            .unwrap();

        let list = data.get_data().get(addr).unwrap().as_list().unwrap().0;

        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Symbol(symbol_value("Value1"))
        );
        assert_eq!(
            data.get_data().get(*list.get(1).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(200))
        );
    }

    #[test]
    fn serialize_new_type_variant_index() {
        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new_with_options(
            &mut data,
            GarnishSerializationOptions::new().variant_name_behavior(VariantNameBehavior::Index),
        );

        let addr = serializer
            .serialize_newtype_variant("MyEnum", 100, "Value1", &200)
            .unwrap();

        let list = data.get_data().get(addr).unwrap().as_list().unwrap().0;

        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(100))
        );
        assert_eq!(
            data.get_data().get(*list.get(1).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(200))
        );
    }
}

#[cfg(test)]
mod compound {
    use serde::Serializer;

    use garnish_lang_simple_data::data::{SimpleData, SimpleNumber};
    use garnish_lang_simple_data::{symbol_value, SimpleGarnishData};
    use crate::GarnishSerializationOptions;

    use crate::serializer::{GarnishDataSerializer, StructBehavior, VariantNameBehavior};

    #[test]
    fn serialize_sequence() {
        use serde::ser::SerializeSeq;

        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let mut serializer = serializer.serialize_seq(None).unwrap();

        serializer.serialize_element(&100).unwrap();
        serializer.serialize_element(&200).unwrap();
        serializer.serialize_element(&300).unwrap();

        let addr = serializer.end().unwrap();

        let list = data.get_data().get(addr).unwrap().as_list().unwrap().0;

        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(100))
        );
        assert_eq!(
            data.get_data().get(*list.get(1).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(200))
        );
        assert_eq!(
            data.get_data().get(*list.get(2).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(300))
        );
    }

    #[test]
    fn serialize_tuple() {
        use serde::ser::SerializeTuple;

        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let mut serializer = serializer.serialize_tuple(3).unwrap();

        serializer.serialize_element(&100).unwrap();
        serializer.serialize_element(&200).unwrap();
        serializer.serialize_element(&300).unwrap();

        let addr = serializer.end().unwrap();

        let list = data.get_data().get(addr).unwrap().as_list().unwrap().0;

        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(100))
        );
        assert_eq!(
            data.get_data().get(*list.get(1).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(200))
        );
        assert_eq!(
            data.get_data().get(*list.get(2).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(300))
        );
    }

    #[test]
    fn serialize_struct_without_name() {
        use serde::ser::SerializeStruct;

        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let mut serializer = serializer.serialize_struct("MyTuple", 3).unwrap();

        serializer.serialize_field("one", &100).unwrap();
        serializer.serialize_field("two", &200).unwrap();
        serializer.serialize_field("three", &300).unwrap();

        let addr = serializer.end().unwrap();

        let list = data.get_data().get(addr).unwrap().as_list().unwrap().0;
        assert_eq!(list.len(), 3);

        let (left, right) = data
            .get_data()
            .get(*list.get(0).unwrap())
            .unwrap()
            .as_pair()
            .unwrap();

        assert_eq!(
            data.get_data().get(left).unwrap(),
            &SimpleData::Symbol(symbol_value("one"))
        );
        assert_eq!(
            data.get_data().get(right).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(100))
        );

        let (left, right) = data
            .get_data()
            .get(*list.get(1).unwrap())
            .unwrap()
            .as_pair()
            .unwrap();

        assert_eq!(
            data.get_data().get(left).unwrap(),
            &SimpleData::Symbol(symbol_value("two"))
        );
        assert_eq!(
            data.get_data().get(right).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(200))
        );

        let (left, right) = data
            .get_data()
            .get(*list.get(2).unwrap())
            .unwrap()
            .as_pair()
            .unwrap();

        assert_eq!(
            data.get_data().get(left).unwrap(),
            &SimpleData::Symbol(symbol_value("three"))
        );
        assert_eq!(
            data.get_data().get(right).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(300))
        );
    }

    #[test]
    fn serialize_struct_with_name() {
        use serde::ser::SerializeStruct;

        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new_with_options(
            &mut data,
            GarnishSerializationOptions::new().struct_typing_behavior(StructBehavior::IncludeTyping),
        );

        let mut serializer = serializer.serialize_tuple_struct("MyStruct", 3).unwrap();

        serializer.serialize_field("one", &100).unwrap();
        serializer.serialize_field("two", &200).unwrap();
        serializer.serialize_field("three", &300).unwrap();

        let addr = serializer.end().unwrap();

        let list = data.get_data().get(addr).unwrap().as_list().unwrap().0;

        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Symbol(symbol_value("MyStruct"))
        );

        let list = data
            .get_data()
            .get(*list.get(1).unwrap())
            .unwrap()
            .as_list()
            .unwrap()
            .0;

        let (left, right) = data
            .get_data()
            .get(*list.get(0).unwrap())
            .unwrap()
            .as_pair()
            .unwrap();

        assert_eq!(
            data.get_data().get(left).unwrap(),
            &SimpleData::Symbol(symbol_value("one"))
        );
        assert_eq!(
            data.get_data().get(right).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(100))
        );

        let (left, right) = data
            .get_data()
            .get(*list.get(1).unwrap())
            .unwrap()
            .as_pair()
            .unwrap();

        assert_eq!(
            data.get_data().get(left).unwrap(),
            &SimpleData::Symbol(symbol_value("two"))
        );
        assert_eq!(
            data.get_data().get(right).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(200))
        );

        let (left, right) = data
            .get_data()
            .get(*list.get(2).unwrap())
            .unwrap()
            .as_pair()
            .unwrap();

        assert_eq!(
            data.get_data().get(left).unwrap(),
            &SimpleData::Symbol(symbol_value("three"))
        );
        assert_eq!(
            data.get_data().get(right).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(300))
        );
    }

    #[test]
    fn serialize_tuple_struct_without_name() {
        use serde::ser::SerializeTupleStruct;

        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let mut serializer = serializer.serialize_tuple_struct("MyTuple", 3).unwrap();

        serializer.serialize_field(&100).unwrap();
        serializer.serialize_field(&200).unwrap();
        serializer.serialize_field(&300).unwrap();

        let addr = serializer.end().unwrap();

        let list = data.get_data().get(addr).unwrap().as_list().unwrap().0;

        assert_eq!(list.len(), 3);
        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(100))
        );
        assert_eq!(
            data.get_data().get(*list.get(1).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(200))
        );
        assert_eq!(
            data.get_data().get(*list.get(2).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(300))
        );
    }

    #[test]
    fn serialize_tuple_struct_with_name() {
        use serde::ser::SerializeTupleStruct;

        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new_with_options(
            &mut data,
            GarnishSerializationOptions::new().struct_typing_behavior(StructBehavior::IncludeTyping),
        );

        let mut serializer = serializer.serialize_tuple_struct("MyTuple", 3).unwrap();

        serializer.serialize_field(&100).unwrap();
        serializer.serialize_field(&200).unwrap();
        serializer.serialize_field(&300).unwrap();

        let addr = serializer.end().unwrap();

        let list = data.get_data().get(addr).unwrap().as_list().unwrap().0;

        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Symbol(symbol_value("MyTuple"))
        );

        let list = data
            .get_data()
            .get(*list.get(1).unwrap())
            .unwrap()
            .as_list()
            .unwrap()
            .0;

        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(100))
        );
        assert_eq!(
            data.get_data().get(*list.get(1).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(200))
        );
        assert_eq!(
            data.get_data().get(*list.get(2).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(300))
        );
    }

    #[test]
    fn serialize_struct_variant_with_full_name() {
        use serde::ser::SerializeStructVariant;

        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let mut serializer = serializer
            .serialize_struct_variant("MyEnum", 100, "MyStruct", 3)
            .unwrap();

        serializer.serialize_field("one", &100).unwrap();
        serializer.serialize_field("two", &200).unwrap();
        serializer.serialize_field("three", &300).unwrap();

        let addr = serializer.end().unwrap();

        let list = data.get_data().get(addr).unwrap().as_list().unwrap().0;

        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Symbol(symbol_value("MyEnum::MyStruct"))
        );

        let list = data
            .get_data()
            .get(*list.get(1).unwrap())
            .unwrap()
            .as_list()
            .unwrap()
            .0;

        let (left, right) = data
            .get_data()
            .get(*list.get(0).unwrap())
            .unwrap()
            .as_pair()
            .unwrap();

        assert_eq!(
            data.get_data().get(left).unwrap(),
            &SimpleData::Symbol(symbol_value("one"))
        );
        assert_eq!(
            data.get_data().get(right).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(100))
        );

        let (left, right) = data
            .get_data()
            .get(*list.get(1).unwrap())
            .unwrap()
            .as_pair()
            .unwrap();

        assert_eq!(
            data.get_data().get(left).unwrap(),
            &SimpleData::Symbol(symbol_value("two"))
        );
        assert_eq!(
            data.get_data().get(right).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(200))
        );

        let (left, right) = data
            .get_data()
            .get(*list.get(2).unwrap())
            .unwrap()
            .as_pair()
            .unwrap();

        assert_eq!(
            data.get_data().get(left).unwrap(),
            &SimpleData::Symbol(symbol_value("three"))
        );
        assert_eq!(
            data.get_data().get(right).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(300))
        );
    }

    #[test]
    fn serialize_struct_variant_with_short_name() {
        use serde::ser::SerializeStructVariant;

        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new_with_options(
            &mut data,
            GarnishSerializationOptions::new().variant_name_behavior(VariantNameBehavior::Short),
        );

        let mut serializer = serializer
            .serialize_struct_variant("MyEnum", 100, "MyStruct", 3)
            .unwrap();

        serializer.serialize_field("one", &100).unwrap();
        serializer.serialize_field("two", &200).unwrap();
        serializer.serialize_field("three", &300).unwrap();

        let addr = serializer.end().unwrap();

        let list = data.get_data().get(addr).unwrap().as_list().unwrap().0;

        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Symbol(symbol_value("MyStruct"))
        );

        let list = data
            .get_data()
            .get(*list.get(1).unwrap())
            .unwrap()
            .as_list()
            .unwrap()
            .0;

        let (left, right) = data
            .get_data()
            .get(*list.get(0).unwrap())
            .unwrap()
            .as_pair()
            .unwrap();

        assert_eq!(
            data.get_data().get(left).unwrap(),
            &SimpleData::Symbol(symbol_value("one"))
        );
        assert_eq!(
            data.get_data().get(right).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(100))
        );

        let (left, right) = data
            .get_data()
            .get(*list.get(1).unwrap())
            .unwrap()
            .as_pair()
            .unwrap();

        assert_eq!(
            data.get_data().get(left).unwrap(),
            &SimpleData::Symbol(symbol_value("two"))
        );
        assert_eq!(
            data.get_data().get(right).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(200))
        );

        let (left, right) = data
            .get_data()
            .get(*list.get(2).unwrap())
            .unwrap()
            .as_pair()
            .unwrap();

        assert_eq!(
            data.get_data().get(left).unwrap(),
            &SimpleData::Symbol(symbol_value("three"))
        );
        assert_eq!(
            data.get_data().get(right).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(300))
        );
    }

    #[test]
    fn serialize_struct_variant_with_index_name() {
        use serde::ser::SerializeStructVariant;

        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new_with_options(
            &mut data,
            GarnishSerializationOptions::new().variant_name_behavior(VariantNameBehavior::Index),
        );

        let mut serializer = serializer
            .serialize_struct_variant("MyEnum", 100, "MyStruct", 3)
            .unwrap();

        serializer.serialize_field("one", &100).unwrap();
        serializer.serialize_field("two", &200).unwrap();
        serializer.serialize_field("three", &300).unwrap();

        let addr = serializer.end().unwrap();

        let list = data.get_data().get(addr).unwrap().as_list().unwrap().0;

        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(100))
        );

        let list = data
            .get_data()
            .get(*list.get(1).unwrap())
            .unwrap()
            .as_list()
            .unwrap()
            .0;

        let (left, right) = data
            .get_data()
            .get(*list.get(0).unwrap())
            .unwrap()
            .as_pair()
            .unwrap();

        assert_eq!(
            data.get_data().get(left).unwrap(),
            &SimpleData::Symbol(symbol_value("one"))
        );
        assert_eq!(
            data.get_data().get(right).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(100))
        );

        let (left, right) = data
            .get_data()
            .get(*list.get(1).unwrap())
            .unwrap()
            .as_pair()
            .unwrap();

        assert_eq!(
            data.get_data().get(left).unwrap(),
            &SimpleData::Symbol(symbol_value("two"))
        );
        assert_eq!(
            data.get_data().get(right).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(200))
        );

        let (left, right) = data
            .get_data()
            .get(*list.get(2).unwrap())
            .unwrap()
            .as_pair()
            .unwrap();

        assert_eq!(
            data.get_data().get(left).unwrap(),
            &SimpleData::Symbol(symbol_value("three"))
        );
        assert_eq!(
            data.get_data().get(right).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(300))
        );
    }

    #[test]
    fn serialize_tuple_variant_with_full_name() {
        use serde::ser::SerializeTupleVariant;

        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let mut serializer = serializer
            .serialize_tuple_variant("MyEnum", 100, "Type1", 3)
            .unwrap();

        serializer.serialize_field(&100).unwrap();
        serializer.serialize_field(&200).unwrap();
        serializer.serialize_field(&300).unwrap();

        let addr = serializer.end().unwrap();

        let list = data.get_data().get(addr).unwrap().as_list().unwrap().0;

        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Symbol(symbol_value("MyEnum::Type1"))
        );

        let list = data
            .get_data()
            .get(*list.get(1).unwrap())
            .unwrap()
            .as_list()
            .unwrap()
            .0;

        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(100))
        );
        assert_eq!(
            data.get_data().get(*list.get(1).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(200))
        );
        assert_eq!(
            data.get_data().get(*list.get(2).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(300))
        );
    }

    #[test]
    fn serialize_tuple_variant_with_short_name() {
        use serde::ser::SerializeTupleVariant;

        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new_with_options(
            &mut data,
            GarnishSerializationOptions::new().variant_name_behavior(VariantNameBehavior::Short),
        );

        let mut serializer = serializer
            .serialize_tuple_variant("MyEnum", 100, "Type1", 3)
            .unwrap();

        serializer.serialize_field(&100).unwrap();
        serializer.serialize_field(&200).unwrap();
        serializer.serialize_field(&300).unwrap();

        let addr = serializer.end().unwrap();

        let list = data.get_data().get(addr).unwrap().as_list().unwrap().0;

        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Symbol(symbol_value("Type1"))
        );

        let list = data
            .get_data()
            .get(*list.get(1).unwrap())
            .unwrap()
            .as_list()
            .unwrap()
            .0;

        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(100))
        );
        assert_eq!(
            data.get_data().get(*list.get(1).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(200))
        );
        assert_eq!(
            data.get_data().get(*list.get(2).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(300))
        );
    }

    #[test]
    fn serialize_tuple_variant_with_index_name() {
        use serde::ser::SerializeTupleVariant;

        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new_with_options(
            &mut data,
            GarnishSerializationOptions::new().variant_name_behavior(VariantNameBehavior::Index),
        );

        let mut serializer = serializer
            .serialize_tuple_variant("MyEnum", 100, "Type1", 3)
            .unwrap();

        serializer.serialize_field(&100).unwrap();
        serializer.serialize_field(&200).unwrap();
        serializer.serialize_field(&300).unwrap();

        let addr = serializer.end().unwrap();

        let list = data.get_data().get(addr).unwrap().as_list().unwrap().0;

        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(100))
        );

        let list = data
            .get_data()
            .get(*list.get(1).unwrap())
            .unwrap()
            .as_list()
            .unwrap()
            .0;

        assert_eq!(
            data.get_data().get(*list.get(0).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(100))
        );
        assert_eq!(
            data.get_data().get(*list.get(1).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(200))
        );
        assert_eq!(
            data.get_data().get(*list.get(2).unwrap()).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(300))
        );
    }

    #[test]
    fn serialize_map() {
        use serde::ser::SerializeMap;

        let mut data = SimpleGarnishData::new();
        let mut serializer = GarnishDataSerializer::new(&mut data);

        let mut serializer = serializer.serialize_map(None).unwrap();

        serializer.serialize_key("one").unwrap();
        serializer.serialize_value(&100).unwrap();
        serializer.serialize_key("two").unwrap();
        serializer.serialize_value(&200).unwrap();
        serializer.serialize_key("three").unwrap();
        serializer.serialize_value(&300).unwrap();

        let addr = serializer.end().unwrap();

        let list = data.get_data().get(addr).unwrap().as_list().unwrap().0;

        let (left, right) = data
            .get_data()
            .get(*list.get(0).unwrap())
            .unwrap()
            .as_pair()
            .unwrap();

        assert_eq!(
            data.get_data().get(left).unwrap(),
            &SimpleData::Symbol(symbol_value("one"))
        );
        assert_eq!(
            data.get_data().get(right).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(100))
        );

        let (left, right) = data
            .get_data()
            .get(*list.get(1).unwrap())
            .unwrap()
            .as_pair()
            .unwrap();

        assert_eq!(
            data.get_data().get(left).unwrap(),
            &SimpleData::Symbol(symbol_value("two"))
        );
        assert_eq!(
            data.get_data().get(right).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(200))
        );

        let (left, right) = data
            .get_data()
            .get(*list.get(2).unwrap())
            .unwrap()
            .as_pair()
            .unwrap();

        assert_eq!(
            data.get_data().get(left).unwrap(),
            &SimpleData::Symbol(symbol_value("three"))
        );
        assert_eq!(
            data.get_data().get(right).unwrap(),
            &SimpleData::Number(SimpleNumber::Integer(300))
        );
    }
}
