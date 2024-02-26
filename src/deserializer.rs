use std::convert::From;

use serde::de::value::StrDeserializer;
use serde::de::{
    DeserializeSeed, EnumAccess, IntoDeserializer, MapAccess, SeqAccess, VariantAccess, Visitor,
};
use serde::Deserializer;

use garnish_lang_traits::{GarnishDataType, GarnishData, TypeConstants};

use crate::error::{wrap_err, GarnishSerializationError};
use crate::GarnishNumberConversions;

pub struct GarnishDataDeserializer<'data, Data>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: Into<usize>,
    Data::Char: Into<char>,
    Data::Byte: Into<u8>,
{
    data: &'data mut Data,
    value_stack: Vec<Data::Size>,
}

impl<'data, Data> GarnishDataDeserializer<'data, Data>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: Into<usize>,
    Data::Char: Into<char>,
    Data::Byte: Into<u8>,
{
    pub fn new(data: &'data mut Data) -> Self {
        let v = data.get_current_value().unwrap_or(Data::Size::zero());
        Self {
            data,
            value_stack: vec![v],
        }
    }

    pub fn new_for_value(data: &'data mut Data, value_addr: Data::Size) -> Self {
        Self {
            data,
            value_stack: vec![value_addr],
        }
    }

    pub fn value(
        &self,
    ) -> Result<(GarnishDataType, Data::Size), GarnishSerializationError<Data>> {
        let a = *self
            .value_stack
            .last()
            .ok_or(GarnishSerializationError::from("No value to deserialize."))?;
        let t = self
            .data
            .get_data_type(a)
            .or_else(|e| Err(GarnishSerializationError::new(e)))?;

        Ok((t, a))
    }

    fn create_symbol_string(
        &mut self,
        a: Data::Size,
    ) -> Result<String, GarnishSerializationError<Data>> {
        // for deserializing identifiers and enums we need to convert symbols to strings
        // may need Garnish Data trait to have a method for direct to string conversion
        let a = self.data.add_char_list_from(a).or_else(wrap_err)?;

        let len = self.data.get_char_list_len(a).or_else(wrap_err)?;
        let mut s = String::with_capacity(len.into());
        let mut i = Data::Size::zero();

        while i < len {
            let c = self
                .data
                .get_char_list_item(a, Data::size_to_number(i))
                .or_else(wrap_err)?;
            s.push(c.into());
            i += Data::Size::one();
        }

        Ok(s)
    }

    fn deserialize_primitive<'de, From, To, V, GetF, VisitF>(
        &self,
        visitor: V,
        get_source: GetF,
        visit_func: VisitF,
        expected_type: GarnishDataType,
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

impl<'data, 'a, Data> Deserializer<'data> for &'a mut GarnishDataDeserializer<'data, Data>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Size: Into<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
    Data::Byte: Into<u8>,
{
    type Error = GarnishSerializationError<Data>;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        unimplemented!()
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        let (t, _a) = self.value()?;

        match t {
            GarnishDataType::True => visitor.visit_bool(true),
            GarnishDataType::False => visitor.visit_bool(false),
            t => Err(GarnishSerializationError::from(
                format!("Expected True or False, found {:?}", t).as_str(),
            )),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_i8,
            GarnishDataType::Number,
        )
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_i16,
            GarnishDataType::Number,
        )
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_i32,
            GarnishDataType::Number,
        )
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_i64,
            GarnishDataType::Number,
        )
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_u8,
            GarnishDataType::Number,
        )
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_u16,
            GarnishDataType::Number,
        )
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_u32,
            GarnishDataType::Number,
        )
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_u64,
            GarnishDataType::Number,
        )
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_f32,
            GarnishDataType::Number,
        )
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_number,
            V::visit_f64,
            GarnishDataType::Number,
        )
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        self.deserialize_primitive(
            visitor,
            Data::get_char,
            V::visit_char,
            GarnishDataType::Char,
        )
    }

    fn deserialize_str<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        Err(GarnishSerializationError::from(
            "Deserialization of &str not supported, use owned type String instead.",
        ))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        let (t, a) = self.value()?;
        match t {
            GarnishDataType::CharList => {
                let len = self.data.get_char_list_len(a).or_else(wrap_err)?;
                let mut s = String::with_capacity(len.into());
                let mut i = Data::Size::zero();

                while i < len {
                    let c = self
                        .data
                        .get_char_list_item(a, Data::size_to_number(i))
                        .or_else(wrap_err)?;
                    s.push(c.into());
                    i += Data::Size::one();
                }

                visitor.visit_string(s)
            }
            // in terms of converting to Rust types, symbols can be treated as Strings if requested
            GarnishDataType::Symbol
            | GarnishDataType::Concatenation
            | GarnishDataType::Slice => {
                // need to create a CharList first
                // may need Garnish Data trait to have a method for direct to string conversion
                let a = self.data.add_char_list_from(a).or_else(wrap_err)?;
                visitor.visit_string(self.create_symbol_string(a)?)
            }
            t => Err(GarnishSerializationError::from(
                format!(
                    "Expected CharList, Symbol, Concatenation or Slice. Found {:?}",
                    t
                )
                .as_str(),
            )),
        }
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        Err(GarnishSerializationError::from(
            "Deserialization of &[u8] not supported, use owned type Vec<u8> instead.",
        ))
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        let (t, a) = self.value()?;
        match t {
            GarnishDataType::ByteList => {
                let len = self.data.get_byte_list_len(a).or_else(wrap_err)?;
                let mut bytes = Vec::with_capacity(len.into());
                let mut i = Data::Size::zero();

                while i < len {
                    let b = self
                        .data
                        .get_byte_list_item(a, Data::size_to_number(i))
                        .or_else(wrap_err)?;
                    bytes.push(b.into());
                    i += Data::Size::one();
                }

                visitor.visit_byte_buf(bytes)
            }
            t => Err(GarnishSerializationError::from(
                format!("Expected ByteList, found {:?}", t).as_str(),
            )),
        }
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        let (t, _a) = self.value()?;
        match t {
            GarnishDataType::Unit => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        let (t, _a) = self.value()?;
        match t {
            GarnishDataType::Unit => visitor.visit_unit(),
            t => Err(GarnishSerializationError::from(
                format!("Expected Unit, found {:?}", t).as_str(),
            )),
        }
    }

    fn deserialize_unit_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        visitor.visit_seq(ListAccessor::new(self)?)
    }

    fn deserialize_tuple<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        visitor.visit_seq(ListAccessor::new_with_max(self, len)?)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        len: usize,
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        visitor.visit_seq(ListAccessor::new_with_max(self, len)?)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        visitor.visit_map(ListAccessor::new(self)?)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        visitor.visit_map(ListAccessor::new(self)?)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        visitor.visit_enum(EnumAccessor::new(self)?)
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        self.deserialize_string(visitor)
    }

    fn deserialize_ignored_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        unimplemented!()
    }
}

struct ListAccessor<'a, 'data, Data>
where
    'data: 'a,
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Size: Into<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
    Data::Byte: Into<u8>,
{
    de: &'a mut GarnishDataDeserializer<'data, Data>,
    items: Vec<Data::Size>,
}

impl<'a, 'data, Data> ListAccessor<'a, 'data, Data>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Size: Into<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
    Data::Byte: Into<u8>,
{
    pub fn new(
        de: &'a mut GarnishDataDeserializer<'data, Data>,
    ) -> Result<Self, GarnishSerializationError<Data>> {
        Self::new_with_max(de, usize::MAX)
    }

    pub fn new_with_max(
        de: &'a mut GarnishDataDeserializer<'data, Data>,
        max: usize,
    ) -> Result<Self, GarnishSerializationError<Data>> {
        let (t, a) = de.value()?;
        let items = match t {
            GarnishDataType::List => gather_list_items(a, de.data)?,
            GarnishDataType::Concatenation => gather_concat_items(a, de.data)?,
            GarnishDataType::Slice => {
                let (list_ref, range_ref) = de.data.get_slice(a).or_else(wrap_err)?;
                let list_type = de.data.get_data_type(list_ref).or_else(wrap_err)?;

                let (start_ref, end_ref) = de.data.get_range(range_ref).or_else(wrap_err)?;
                let (start, end): (usize, usize) = (
                    de.data.get_number(start_ref).or_else(wrap_err)?.into(),
                    de.data.get_number(end_ref).or_else(wrap_err)?.into(),
                );

                let items = match list_type {
                    GarnishDataType::List => gather_list_items(list_ref, de.data)?,
                    GarnishDataType::Concatenation => gather_concat_items(list_ref, de.data)?,
                    t => Err(GarnishSerializationError::from(
                        format!("{:?} Slice cannot be converted to sequence.", t).as_str(),
                    ))?,
                };

                if end < start {
                    vec![]
                } else {
                    // Ranges are stored in garnish data as inclusive on both ends
                    // adding 1 to the dif of end and start will include both
                    let count = end - start + 1;
                    items
                        .iter()
                        .skip(start)
                        .take(count)
                        .map(|i| *i)
                        .collect::<Vec<Data::Size>>()
                }
            }
            // Imply list of length 1 for all other types
            _ => vec![a],
        };

        if items.len() > max {
            return Err(GarnishSerializationError::from(
                format!(
                    "Cannot deserialize list like value. Expected maximum of {} items, found {}",
                    max,
                    items.len()
                )
                .as_str(),
            ));
        }

        Ok(Self {
            de,
            // reverse so items can be popped in order
            items: items.into_iter().rev().collect(),
        })
    }
}

fn gather_concat_items<Data: GarnishData>(
    concat_ref: Data::Size,
    data: &Data,
) -> Result<Vec<Data::Size>, GarnishSerializationError<Data>>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Size: Into<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
    Data::Byte: Into<u8>,
{
    let mut items = vec![];
    let mut cat_stack = vec![concat_ref];
    while !cat_stack.is_empty() {
        let current = match cat_stack.pop() {
            Some(v) => v,
            None => unreachable!("Empty stack when converting a concatenation."),
        };

        match data.get_data_type(current).or_else(wrap_err)? {
            GarnishDataType::Concatenation => {
                let (left, right) = data.get_concatenation(current).or_else(wrap_err)?;
                cat_stack.push(right);
                cat_stack.push(left);
            }
            _ => items.push(current),
        }
    }

    Ok(items)
}

fn gather_list_items<Data: GarnishData>(
    list_ref: Data::Size,
    data: &Data,
) -> Result<Vec<Data::Size>, GarnishSerializationError<Data>>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Size: Into<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
    Data::Byte: Into<u8>,
{
    let len = data.get_list_len(list_ref).or_else(wrap_err)?;
    let mut i = Data::Size::zero();
    let mut items = vec![];
    while i < len {
        let list_item = data
            .get_list_item(list_ref, Data::size_to_number(i))
            .or_else(wrap_err)?;
        items.push(list_item);
        i += Data::Size::one();
    }

    Ok(items)
}

impl<'a, 'data, Data> SeqAccess<'data> for ListAccessor<'a, 'data, Data>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Size: Into<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
    Data::Byte: Into<u8>,
{
    type Error = GarnishSerializationError<Data>;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error>
    where
        T: DeserializeSeed<'data>,
    {
        if let Some(item) = self.items.pop() {
            self.de.value_stack.push(item);

            let r = seed.deserialize(&mut *self.de).map(Some);

            // done with list item
            self.de.value_stack.pop();

            r
        } else {
            Ok(None)
        }
    }
}

impl<'a, 'data, Data> MapAccess<'data> for ListAccessor<'a, 'data, Data>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Size: Into<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
    Data::Byte: Into<u8>,
{
    type Error = GarnishSerializationError<Data>;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error>
    where
        K: DeserializeSeed<'data>,
    {
        if let Some(item) = self.items.pop() {
            let (key, value) = self.de.data.get_pair(item).or_else(wrap_err)?;
            self.de.value_stack.push(key);

            let r = seed.deserialize(&mut *self.de).map(Some);

            // done with key
            self.de.value_stack.pop();

            // set up value for next_value_seed
            self.de.value_stack.push(value);

            r
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value, Self::Error>
    where
        V: DeserializeSeed<'data>,
    {
        // won't be called unless next returns non-None value
        // so don't need to check length again
        // value addr was set up by next_key_seed
        // just need to deserialize
        let r = seed.deserialize(&mut *self.de);
        // remove value addr
        self.de.value_stack.pop();

        r
    }
}

struct EnumAccessor<'a, 'data, Data>
where
    'data: 'a,
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Size: Into<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
    Data::Byte: Into<u8>,
{
    de: &'a mut GarnishDataDeserializer<'data, Data>,
}

impl<'a, 'data, Data> EnumAccessor<'a, 'data, Data>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Size: Into<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
    Data::Byte: Into<u8>,
{
    pub fn new(
        de: &'a mut GarnishDataDeserializer<'data, Data>,
    ) -> Result<Self, GarnishSerializationError<Data>> {
        Ok(Self { de })
    }
}

impl<'a, 'data, Data> EnumAccess<'data> for EnumAccessor<'a, 'data, Data>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Size: Into<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
    Data::Byte: Into<u8>,
{
    type Error = GarnishSerializationError<Data>;
    type Variant = Self;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant), Self::Error>
    where
        V: DeserializeSeed<'data>,
    {
        let (t, a) = self.de.value()?;
        let sym_a = match t {
            GarnishDataType::List => {
                let first = self
                    .de
                    .data
                    .get_list_item(a, Data::Number::zero())
                    .or_else(wrap_err)?;

                // need to push variant value to stack for access after identification
                let second = self
                    .de
                    .data
                    .get_list_item(a, Data::Number::one())
                    .or_else(wrap_err)?;

                self.de.value_stack.push(second);

                first
            }
            GarnishDataType::Symbol => a,
            _ => Err(GarnishSerializationError::from(
                format!("Expected List or Symbol for variant, found {:?}", t).as_str(),
            ))?,
        };

        let sym = self.de.create_symbol_string(sym_a)?;
        // stored as full name should be split with following pattern
        // resulting in 2 elements
        let mut parts = sym.split("::");
        parts.next(); // drop first value, don't need currently
        let enum_part = parts.next().ok_or_else(|| {
            GarnishSerializationError::from(
                format!("Could not get enum value from symbols string {:?}", sym).as_str(),
            )
        })?;

        let deserializer: StrDeserializer<'_, GarnishSerializationError<Data>> =
            enum_part.into_deserializer();
        let variant_value = seed.deserialize(deserializer)?;

        Ok((variant_value, self))
    }
}

impl<'a, 'data, Data> VariantAccess<'data> for EnumAccessor<'a, 'data, Data>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
    Data::Size: From<usize>,
    Data::Size: Into<usize>,
    Data::Char: From<char>,
    Data::Char: Into<char>,
    Data::Byte: From<u8>,
    Data::Byte: Into<u8>,
{
    type Error = GarnishSerializationError<Data>;
    // unit variants are stored simply as a symbol
    // all other variants are store as a list
    // with the full variant name as the first item
    // and the data, if any, as the second item

    fn unit_variant(self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value, Self::Error>
    where
        T: DeserializeSeed<'data>,
    {
        seed.deserialize(self.de)
    }

    fn tuple_variant<V>(self, len: usize, visitor: V) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        self.de.deserialize_tuple(len, visitor)
    }

    fn struct_variant<V>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Self::Error>
    where
        V: Visitor<'data>,
    {
        self.de.deserialize_struct("", fields, visitor)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fmt::{Debug, Formatter};
    use std::marker::PhantomData;

    use serde::de::{DeserializeOwned, Error, Visitor};
    use serde::{Deserialize, Deserializer};

    use garnish_lang_simple_data::data::SimpleNumber;
    use garnish_lang_simple_data::{DataError, SimpleGarnishData};
    use garnish_lang_traits::GarnishData;

    use crate::deserializer::GarnishDataDeserializer;
    use crate::error::GarnishSerializationError;

    fn deserialize<SetupF, Type>(
        setup: SetupF,
    ) -> Result<Type, GarnishSerializationError<SimpleGarnishData>>
    where
        SetupF: FnOnce(&mut SimpleGarnishData) -> Result<usize, DataError>,
        Type: DeserializeOwned + PartialEq + Debug,
    {
        let mut data = SimpleGarnishData::new();
        let addr = setup(&mut data).unwrap();
        data.push_value_stack(addr).unwrap();

        let mut deserializer = GarnishDataDeserializer::new(&mut data);

        Type::deserialize(&mut deserializer)
    }

    fn assert_deserializes<SetupF, Type>(setup: SetupF, expected_value: Type)
    where
        SetupF: FnOnce(&mut SimpleGarnishData) -> Result<usize, DataError>,
        Type: DeserializeOwned + PartialEq + Debug,
    {
        let v = deserialize::<SetupF, Type>(setup);
        match v {
            Ok(v) => assert_eq!(v, expected_value),
            Err(e) => assert!(false, "{}", format!("{:?} - {:?}", e.error(), e.message())),
        }
    }

    fn assert_fails<SetupF, Type>(setup: SetupF)
    where
        SetupF: FnOnce(&mut SimpleGarnishData) -> Result<usize, DataError>,
        Type: DeserializeOwned + PartialEq + Debug,
    {
        assert!(deserialize::<SetupF, Type>(setup).is_err());
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

    #[test]
    fn deserialize_string_from_concatenation() {
        assert_deserializes(
            |data| {
                let s1 = data.parse_add_char_list("abcd").unwrap();
                let s2 = data.parse_add_char_list("efgh").unwrap();
                let s3 = data.parse_add_char_list("ijkl").unwrap();

                let cat1 = data.add_concatenation(s1, s2).unwrap();
                data.add_concatenation(cat1, s3)
            },
            String::from("abcdefghijkl"),
        );
    }

    #[test]
    fn deserialize_string_from_slice() {
        assert_deserializes(
            |data| {
                let s = data.parse_add_char_list("abcd").unwrap();
                let start = data.add_number(SimpleNumber::Integer(1)).unwrap();
                let end = data.add_number(SimpleNumber::Integer(2)).unwrap();
                let range = data.add_range(start, end).unwrap();

                data.add_slice(s, range)
            },
            String::from("bc"),
        );
    }

    // cannot currently be implemented
    // #[test]
    // fn deserialize_bytes() {
    //     assert_deserializes(|data| {
    //         data.parse_add_byte_list("abcd")
    //     }, &['data' as u8, 'b' as u8, 'c' as u8, 'd' as u8]);
    // }

    // regular vec was calling sequence deserialization
    // made this one to ensure byte buf functions are called
    #[derive(Debug, Clone, PartialEq)]
    struct SomeBytes {
        bytes: Vec<u8>,
    }

    impl<'de> Deserialize<'de> for SomeBytes {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct SomeBytesVisitor;
            impl<'de> Visitor<'de> for SomeBytesVisitor {
                type Value = SomeBytes;

                fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
                    formatter.write_str("Expecting vec of bytes.")
                }

                fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
                where
                    E: Error,
                {
                    Ok(SomeBytes { bytes: v })
                }
            }

            deserializer.deserialize_byte_buf(SomeBytesVisitor)
        }
    }

    #[test]
    fn deserialize_byte_buf() {
        assert_deserializes(
            |data| data.parse_add_byte_list("abcd"),
            SomeBytes {
                bytes: vec!['a' as u8, 'b' as u8, 'c' as u8, 'd' as u8],
            },
        );
    }

    #[test]
    fn deserialize_option_some() {
        assert_deserializes(
            |data| data.add_number(SimpleNumber::Integer(100)),
            Some(100),
        );
    }

    #[test]
    fn deserialize_option_none() {
        assert_deserializes(|data| data.add_unit(), None::<i32>);
    }

    #[test]
    fn deserialize_unit() {
        assert_deserializes(|data| data.add_unit(), ());
    }

    #[test]
    fn deserialize_unit_struct() {
        assert_deserializes(|data| data.add_unit(), PhantomData::<i32>);
    }

    #[derive(Deserialize, Debug, PartialEq)]
    struct SomeNumber(i32);

    #[test]
    fn deserialize_newtype_struct() {
        assert_deserializes(
            |data| data.add_number(SimpleNumber::Integer(100)),
            SomeNumber(100),
        );
    }

    #[test]
    fn deserialize_seq() {
        assert_deserializes(
            |data| {
                let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
                let num2 = data.add_number(SimpleNumber::Integer(200)).unwrap();
                let num3 = data.add_number(SimpleNumber::Integer(300)).unwrap();
                data.start_list(3).unwrap();
                data.add_to_list(num1, false).unwrap();
                data.add_to_list(num2, false).unwrap();
                data.add_to_list(num3, false).unwrap();
                data.end_list()
            },
            vec![100, 200, 300],
        );
    }

    #[test]
    fn deserialize_seq_of_1_from_non_list() {
        assert_deserializes(
            |data| data.add_number(SimpleNumber::Integer(100)),
            vec![100],
        );
    }

    #[test]
    fn deserialize_seq_from_concatenation() {
        assert_deserializes(
            |data| {
                let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
                let num2 = data.add_number(SimpleNumber::Integer(200)).unwrap();
                let num3 = data.add_number(SimpleNumber::Integer(300)).unwrap();

                let con1 = data.add_concatenation(num1, num2).unwrap();
                data.add_concatenation(con1, num3)
            },
            vec![100, 200, 300],
        );
    }

    #[test]
    fn deserialize_seq_from_concatenation_slice() {
        assert_deserializes(
            |data| {
                let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
                let num2 = data.add_number(SimpleNumber::Integer(200)).unwrap();
                let num3 = data.add_number(SimpleNumber::Integer(300)).unwrap();

                let con1 = data.add_concatenation(num1, num2).unwrap();
                let con2 = data.add_concatenation(con1, num3).unwrap();

                let start = data.add_number(SimpleNumber::Integer(1)).unwrap();
                let end = data.add_number(SimpleNumber::Integer(2)).unwrap();
                let range = data.add_range(start, end).unwrap();

                data.add_slice(con2, range)
            },
            vec![200, 300],
        );
    }

    #[test]
    fn deserialize_seq_from_list_slice() {
        assert_deserializes(
            |data| {
                let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
                let num2 = data.add_number(SimpleNumber::Integer(200)).unwrap();
                let num3 = data.add_number(SimpleNumber::Integer(300)).unwrap();
                data.start_list(3).unwrap();
                data.add_to_list(num1, false).unwrap();
                data.add_to_list(num2, false).unwrap();
                data.add_to_list(num3, false).unwrap();
                let list = data.end_list().unwrap();

                let start = data.add_number(SimpleNumber::Integer(1)).unwrap();
                let end = data.add_number(SimpleNumber::Integer(2)).unwrap();
                let range = data.add_range(start, end).unwrap();

                data.add_slice(list, range)
            },
            vec![200, 300],
        );
    }

    #[test]
    fn deserialize_seq_from_list_slice_one_length_range() {
        assert_deserializes(
            |data| {
                let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
                let num2 = data.add_number(SimpleNumber::Integer(200)).unwrap();
                let num3 = data.add_number(SimpleNumber::Integer(300)).unwrap();
                data.start_list(3).unwrap();
                data.add_to_list(num1, false).unwrap();
                data.add_to_list(num2, false).unwrap();
                data.add_to_list(num3, false).unwrap();
                let list = data.end_list().unwrap();

                let start = data.add_number(SimpleNumber::Integer(1)).unwrap();
                let end = data.add_number(SimpleNumber::Integer(1)).unwrap();
                let range = data.add_range(start, end).unwrap();

                data.add_slice(list, range)
            },
            vec![200],
        );
    }

    #[test]
    fn deserialize_seq_of_seq() {
        assert_deserializes(
            |data| {
                let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
                let num2 = data.add_number(SimpleNumber::Integer(200)).unwrap();
                let num3 = data.add_number(SimpleNumber::Integer(300)).unwrap();
                data.start_list(3).unwrap();
                data.add_to_list(num1, false).unwrap();
                data.add_to_list(num2, false).unwrap();
                data.add_to_list(num3, false).unwrap();
                let list1 = data.end_list().unwrap();

                let num1 = data.add_number(SimpleNumber::Integer(400)).unwrap();
                let num2 = data.add_number(SimpleNumber::Integer(500)).unwrap();
                let num3 = data.add_number(SimpleNumber::Integer(600)).unwrap();
                data.start_list(3).unwrap();
                data.add_to_list(num1, false).unwrap();
                data.add_to_list(num2, false).unwrap();
                data.add_to_list(num3, false).unwrap();
                let list2 = data.end_list().unwrap();

                data.start_list(2).unwrap();
                data.add_to_list(list1, false).unwrap();
                data.add_to_list(list2, false).unwrap();
                data.end_list()
            },
            vec![vec![100, 200, 300], vec![400, 500, 600]],
        );
    }

    #[test]
    fn deserialize_tuple() {
        assert_deserializes(
            |data| {
                let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
                let num2 = data.add_number(SimpleNumber::Integer(200)).unwrap();
                let num3 = data.add_number(SimpleNumber::Integer(300)).unwrap();
                data.start_list(3).unwrap();
                data.add_to_list(num1, false).unwrap();
                data.add_to_list(num2, false).unwrap();
                data.add_to_list(num3, false).unwrap();
                data.end_list()
            },
            (100, 200, 300),
        );
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct SomeNumbers(i32, i32, i32);

    #[test]
    fn deserialize_tuple_struct() {
        assert_deserializes(
            |data| {
                let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
                let num2 = data.add_number(SimpleNumber::Integer(200)).unwrap();
                let num3 = data.add_number(SimpleNumber::Integer(300)).unwrap();
                data.start_list(3).unwrap();
                data.add_to_list(num1, false).unwrap();
                data.add_to_list(num2, false).unwrap();
                data.add_to_list(num3, false).unwrap();
                data.end_list()
            },
            SomeNumbers(100, 200, 300),
        );
    }

    #[test]
    fn deserialize_tuple_to_few() {
        assert_fails::<_, (i32, i32, i32)>(|data| {
            let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
            let num2 = data.add_number(SimpleNumber::Integer(200)).unwrap();
            data.start_list(2).unwrap();
            data.add_to_list(num1, false).unwrap();
            data.add_to_list(num2, false).unwrap();
            data.end_list()
        });
    }

    #[test]
    fn deserialize_tuple_to_many() {
        assert_fails::<_, (i32, i32, i32)>(|data| {
            let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
            let num2 = data.add_number(SimpleNumber::Integer(200)).unwrap();
            let num3 = data.add_number(SimpleNumber::Integer(300)).unwrap();
            let num4 = data.add_number(SimpleNumber::Integer(400)).unwrap();
            data.start_list(4).unwrap();
            data.add_to_list(num1, false).unwrap();
            data.add_to_list(num2, false).unwrap();
            data.add_to_list(num3, false).unwrap();
            data.add_to_list(num4, false).unwrap();
            data.end_list()
        });
    }

    #[test]
    fn deserialize_map() {
        let mut expected = HashMap::new();
        expected.insert("one".to_string(), 100);
        expected.insert("two".to_string(), 200);
        expected.insert("three".to_string(), 300);

        assert_deserializes(
            |data| {
                let sym1 = data.parse_add_symbol("one").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
                let pair1 = data.add_pair((sym1, num1)).unwrap();

                let sym1 = data.parse_add_symbol("two").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(200)).unwrap();
                let pair2 = data.add_pair((sym1, num1)).unwrap();

                let sym1 = data.parse_add_symbol("three").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(300)).unwrap();
                let pair3 = data.add_pair((sym1, num1)).unwrap();

                data.start_list(3).unwrap();
                data.add_to_list(pair1, true).unwrap();
                data.add_to_list(pair2, true).unwrap();
                data.add_to_list(pair3, true).unwrap();
                data.end_list()
            },
            expected,
        );
    }

    #[test]
    fn deserialize_map_from_list_slice() {
        let mut expected = HashMap::new();
        expected.insert("two".to_string(), 200);
        expected.insert("three".to_string(), 300);
        expected.insert("four".to_string(), 400);

        assert_deserializes(
            |data| {
                let sym1 = data.parse_add_symbol("one").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
                let pair1 = data.add_pair((sym1, num1)).unwrap();

                let sym1 = data.parse_add_symbol("two").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(200)).unwrap();
                let pair2 = data.add_pair((sym1, num1)).unwrap();

                let sym1 = data.parse_add_symbol("three").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(300)).unwrap();
                let pair3 = data.add_pair((sym1, num1)).unwrap();

                let sym1 = data.parse_add_symbol("four").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(400)).unwrap();
                let pair4 = data.add_pair((sym1, num1)).unwrap();

                data.start_list(3).unwrap();
                data.add_to_list(pair1, true).unwrap();
                data.add_to_list(pair2, true).unwrap();
                data.add_to_list(pair3, true).unwrap();
                data.add_to_list(pair4, true).unwrap();
                let list = data.end_list().unwrap();

                let start = data.add_number(SimpleNumber::Integer(1)).unwrap();
                let end = data.add_number(SimpleNumber::Integer(3)).unwrap();
                let range = data.add_range(start, end).unwrap();

                data.add_slice(list, range)
            },
            expected,
        );
    }

    #[test]
    fn deserialize_map_from_concatenation() {
        let mut expected = HashMap::new();
        expected.insert("two".to_string(), 200);
        expected.insert("three".to_string(), 300);
        expected.insert("four".to_string(), 400);

        assert_deserializes(
            |data| {
                let sym1 = data.parse_add_symbol("one").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
                let pair1 = data.add_pair((sym1, num1)).unwrap();

                let sym1 = data.parse_add_symbol("two").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(200)).unwrap();
                let pair2 = data.add_pair((sym1, num1)).unwrap();

                let sym1 = data.parse_add_symbol("three").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(300)).unwrap();
                let pair3 = data.add_pair((sym1, num1)).unwrap();

                let sym1 = data.parse_add_symbol("four").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(400)).unwrap();
                let pair4 = data.add_pair((sym1, num1)).unwrap();

                let cat1 = data.add_concatenation(pair1, pair2).unwrap();
                let cat2 = data.add_concatenation(cat1, pair3).unwrap();
                let cat3 = data.add_concatenation(cat2, pair4).unwrap();

                let start = data.add_number(SimpleNumber::Integer(1)).unwrap();
                let end = data.add_number(SimpleNumber::Integer(3)).unwrap();
                let range = data.add_range(start, end).unwrap();

                data.add_slice(cat3, range)
            },
            expected,
        );
    }

    #[test]
    fn deserialize_map_from_concatenation_slice() {
        let mut expected = HashMap::new();
        expected.insert("one".to_string(), 100);
        expected.insert("two".to_string(), 200);
        expected.insert("three".to_string(), 300);

        assert_deserializes(
            |data| {
                let sym1 = data.parse_add_symbol("one").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
                let pair1 = data.add_pair((sym1, num1)).unwrap();

                let sym1 = data.parse_add_symbol("two").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(200)).unwrap();
                let pair2 = data.add_pair((sym1, num1)).unwrap();

                let sym1 = data.parse_add_symbol("three").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(300)).unwrap();
                let pair3 = data.add_pair((sym1, num1)).unwrap();

                let cat1 = data.add_concatenation(pair1, pair2).unwrap();
                data.add_concatenation(cat1, pair3)
            },
            expected,
        );
    }

    #[derive(Debug, PartialEq, Deserialize)]
    struct SomeStruct {
        one: i32,
        two: i32,
        three: i32,
    }

    fn add_some_struct(data: &mut SimpleGarnishData) -> Result<usize, DataError> {
        let sym1 = data.parse_add_symbol("one").unwrap();
        let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
        let pair1 = data.add_pair((sym1, num1)).unwrap();

        let sym1 = data.parse_add_symbol("two").unwrap();
        let num1 = data.add_number(SimpleNumber::Integer(200)).unwrap();
        let pair2 = data.add_pair((sym1, num1)).unwrap();

        let sym1 = data.parse_add_symbol("three").unwrap();
        let num1 = data.add_number(SimpleNumber::Integer(300)).unwrap();
        let pair3 = data.add_pair((sym1, num1)).unwrap();

        data.start_list(3).unwrap();
        data.add_to_list(pair1, true).unwrap();
        data.add_to_list(pair2, true).unwrap();
        data.add_to_list(pair3, true).unwrap();
        data.end_list()
    }

    fn add_some_struct_as_concat(data: &mut SimpleGarnishData) -> Result<usize, DataError> {
        let sym1 = data.parse_add_symbol("one").unwrap();
        let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
        let pair1 = data.add_pair((sym1, num1)).unwrap();

        let sym1 = data.parse_add_symbol("two").unwrap();
        let num1 = data.add_number(SimpleNumber::Integer(200)).unwrap();
        let pair2 = data.add_pair((sym1, num1)).unwrap();

        let sym1 = data.parse_add_symbol("three").unwrap();
        let num1 = data.add_number(SimpleNumber::Integer(300)).unwrap();
        let pair3 = data.add_pair((sym1, num1)).unwrap();

        let cat1 = data.add_concatenation(pair1, pair2).unwrap();
        data.add_concatenation(cat1, pair3)
    }

    #[test]
    fn deserialize_struct() {
        assert_deserializes(
            add_some_struct,
            SomeStruct {
                one: 100,
                two: 200,
                three: 300,
            },
        );
    }

    #[test]
    fn deserialize_struct_from_list_slice() {
        assert_deserializes(
            |data| {
                let sym1 = data.parse_add_symbol("zero").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(0)).unwrap();
                let pair1 = data.add_pair((sym1, num1)).unwrap();

                let sym1 = data.parse_add_symbol("one").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
                let pair2 = data.add_pair((sym1, num1)).unwrap();

                let sym1 = data.parse_add_symbol("two").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(200)).unwrap();
                let pair3 = data.add_pair((sym1, num1)).unwrap();

                let sym1 = data.parse_add_symbol("three").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(300)).unwrap();
                let pair4 = data.add_pair((sym1, num1)).unwrap();

                data.start_list(3).unwrap();
                data.add_to_list(pair1, true).unwrap();
                data.add_to_list(pair2, true).unwrap();
                data.add_to_list(pair3, true).unwrap();
                data.add_to_list(pair4, true).unwrap();
                let list = data.end_list().unwrap();

                let start = data.add_number(SimpleNumber::Integer(1)).unwrap();
                let end = data.add_number(SimpleNumber::Integer(3)).unwrap();
                let range = data.add_range(start, end).unwrap();

                data.add_slice(list, range)
            },
            SomeStruct {
                one: 100,
                two: 200,
                three: 300,
            },
        );
    }

    #[test]
    fn deserialize_struct_from_concatenation() {
        assert_deserializes(
            add_some_struct_as_concat,
            SomeStruct {
                one: 100,
                two: 200,
                three: 300,
            },
        );
    }

    #[test]
    fn deserialize_struct_from_concatenation_slice() {
        assert_deserializes(
            |data| {
                let sym1 = data.parse_add_symbol("zero").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(0)).unwrap();
                let pair1 = data.add_pair((sym1, num1)).unwrap();

                let sym1 = data.parse_add_symbol("one").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
                let pair2 = data.add_pair((sym1, num1)).unwrap();

                let sym1 = data.parse_add_symbol("two").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(200)).unwrap();
                let pair3 = data.add_pair((sym1, num1)).unwrap();

                let sym1 = data.parse_add_symbol("three").unwrap();
                let num1 = data.add_number(SimpleNumber::Integer(300)).unwrap();
                let pair4 = data.add_pair((sym1, num1)).unwrap();

                let cat1 = data.add_concatenation(pair1, pair2).unwrap();
                let cat2 = data.add_concatenation(cat1, pair3).unwrap();
                let cat3 = data.add_concatenation(cat2, pair4).unwrap();

                let start = data.add_number(SimpleNumber::Integer(1)).unwrap();
                let end = data.add_number(SimpleNumber::Integer(3)).unwrap();
                let range = data.add_range(start, end).unwrap();

                data.add_slice(cat3, range)
            },
            SomeStruct {
                one: 100,
                two: 200,
                three: 300,
            },
        );
    }

    #[derive(Debug, PartialEq, Deserialize)]
    enum SomeEnum {
        SomeUnitVariant,
        SomeNewTypeVariant(i32),
        SomeTupleVariant(i32, i32),
        SomeStructVariant { one: i32, two: i32, three: i32 },
    }

    #[test]
    fn deserialize_unit_variant() {
        assert_deserializes(
            |data| data.parse_add_symbol("SomeEnum::SomeUnitVariant"),
            SomeEnum::SomeUnitVariant,
        );
    }

    #[test]
    fn deserialize_newtype_variant() {
        assert_deserializes(
            |data| {
                let value = data.add_number(SimpleNumber::Integer(100)).unwrap();

                let variant = data
                    .parse_add_symbol("SomeEnum::SomeNewTypeVariant")
                    .unwrap();

                data.start_list(2).unwrap();
                data.add_to_list(variant, false).unwrap();
                data.add_to_list(value, false).unwrap();
                data.end_list()
            },
            SomeEnum::SomeNewTypeVariant(100),
        );
    }

    #[test]
    fn deserialize_tuple_variant() {
        assert_deserializes(
            |data| {
                let num1 = data.add_number(SimpleNumber::Integer(100)).unwrap();
                let num2 = data.add_number(SimpleNumber::Integer(200)).unwrap();
                data.start_list(2).unwrap();
                data.add_to_list(num1, false).unwrap();
                data.add_to_list(num2, false).unwrap();
                let value = data.end_list().unwrap();

                let variant = data.parse_add_symbol("SomeEnum::SomeTupleVariant").unwrap();

                data.start_list(2).unwrap();
                data.add_to_list(variant, false).unwrap();
                data.add_to_list(value, false).unwrap();
                data.end_list()
            },
            SomeEnum::SomeTupleVariant(100, 200),
        );
    }

    #[test]
    fn deserialize_struct_variant() {
        assert_deserializes(
            |data| {
                let value = add_some_struct(data).unwrap();

                let variant = data
                    .parse_add_symbol("SomeEnum::SomeStructVariant")
                    .unwrap();

                data.start_list(2).unwrap();
                data.add_to_list(variant, false).unwrap();
                data.add_to_list(value, false).unwrap();
                data.end_list()
            },
            SomeEnum::SomeStructVariant {
                one: 100,
                two: 200,
                three: 300,
            },
        );
    }
}
