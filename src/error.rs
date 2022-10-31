use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use garnish_traits::GarnishLangRuntimeData;
use crate::serializer::GarnishNumberConversions;

pub struct GarnishSerializationError<Data>
    where
        Data: GarnishLangRuntimeData,
        Data::Number: GarnishNumberConversions,
        Data::Size: From<usize>,
        Data::Char: From<char>,
        Data::Byte: From<u8>,
{
    message: Option<String>,
    err: Option<Data::Error>,
}

impl<Data> GarnishSerializationError<Data>
    where
        Data: GarnishLangRuntimeData,
        Data::Number: GarnishNumberConversions,
        Data::Size: From<usize>,
        Data::Char: From<char>,
        Data::Byte: From<u8>,
{
    pub fn new(err: Data::Error) -> Self {
        Self { message: None, err: Some(err) }
    }

    pub fn message(&self) -> Option<&String> {
        self.message.as_ref()
    }

    pub fn error(&self) -> Option<&Data::Error> {
        self.err.as_ref()
    }
}

impl<Data> From<&str> for GarnishSerializationError<Data>
    where
        Data: GarnishLangRuntimeData,
        Data::Number: GarnishNumberConversions,
        Data::Size: From<usize>,
        Data::Char: From<char>,
        Data::Byte: From<u8>,
{
    fn from(s: &str) -> Self {
        Self { message: Some(s.to_string()), err: None }
    }
}

impl<Data> Debug for GarnishSerializationError<Data>
    where
        Data: GarnishLangRuntimeData,
        Data::Number: GarnishNumberConversions,
        Data::Size: From<usize>,
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
        Data::Size: From<usize>,
        Data::Char: From<char>,
        Data::Byte: From<u8>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self.err).as_str())
    }
}

impl<Data> Error for GarnishSerializationError<Data>
    where
        Data: GarnishLangRuntimeData,
        Data::Number: GarnishNumberConversions,
        Data::Size: From<usize>,
        Data::Char: From<char>,
        Data::Byte: From<u8>,
{
}

impl<Data> serde::ser::Error for GarnishSerializationError<Data>
    where
        Data: GarnishLangRuntimeData,
        Data::Number: GarnishNumberConversions,
        Data::Size: From<usize>,
        Data::Char: From<char>,
        Data::Byte: From<u8>,
{
    fn custom<T>(msg: T) -> Self
        where
            T: Display,
    {
        Self {
            message: Some(format!("{}", msg)),
            err: None
        }
    }
}

impl<Data> serde::de::Error for GarnishSerializationError<Data>
    where
        Data: GarnishLangRuntimeData,
        Data::Number: GarnishNumberConversions,
        Data::Size: From<usize>,
        Data::Char: From<char>,
        Data::Byte: From<u8>,
{
    fn custom<T>(msg: T) -> Self
        where
            T: Display,
    {
        Self {
            message: Some(format!("{}", msg)),
            err: None
        }
    }
}