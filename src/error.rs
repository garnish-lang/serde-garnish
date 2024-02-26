use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

use garnish_lang_traits::GarnishData;

use crate::traits::GarnishNumberConversions;

pub struct GarnishSerializationError<Data>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
{
    message: Option<String>,
    err: Option<Data::Error>,
}

impl<Data> GarnishSerializationError<Data>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
{
    pub fn new(err: Data::Error) -> Self {
        Self {
            message: None,
            err: Some(err),
        }
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
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
{
    fn from(s: &str) -> Self {
        Self {
            message: Some(s.to_string()),
            err: None,
        }
    }
}

impl<Data> Debug for GarnishSerializationError<Data>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self.err).as_str())
    }
}

impl<Data> Display for GarnishSerializationError<Data>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self.err).as_str())
    }
}

impl<Data> Error for GarnishSerializationError<Data>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
{
}

impl<Data> serde::ser::Error for GarnishSerializationError<Data>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
{
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self {
            message: Some(format!("{}", msg)),
            err: None,
        }
    }
}

impl<Data> serde::de::Error for GarnishSerializationError<Data>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
{
    fn custom<T>(msg: T) -> Self
    where
        T: Display,
    {
        Self {
            message: Some(format!("{}", msg)),
            err: None,
        }
    }
}

pub fn wrap_err<V, Data>(e: Data::Error) -> Result<V, GarnishSerializationError<Data>>
where
    Data: GarnishData,
    Data::Number: GarnishNumberConversions,
{
    Err(GarnishSerializationError::new(e))
}
