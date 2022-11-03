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
+ Into<i8>
+ Into<i16>
+ Into<i32>
+ Into<i64>
+ Into<u8>
+ Into<u16>
+ Into<u32>
+ Into<u64>
+ Into<f32>
+ Into<f64>
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
    + Into<i8>
    + Into<i16>
    + Into<i32>
    + Into<i64>
    + Into<u8>
    + Into<u16>
    + Into<u32>
    + Into<u64>
    + Into<f32>
    + Into<f64>
{
}