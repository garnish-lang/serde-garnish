/// ## Serialization Type Conversions
/// Mapping from Serde data model to Garnish value
/// | *Serde Type*                          | *Example*                         | *Garnish Value*               | *Garnish Type*                |
/// | i8, i16, i32, i64, u8, u16, u32, u64  | 100                               | 100                           | Number                        |
/// | f32, f64                              | 5.0                               | 5.0                           | Number                        |
/// | bool                                  | true                              | $?                            | True, False                   |
/// | char                                  | 'a'                               | "a"                           | Char                          |
/// | String                                | "abcd"                            | "abcd"                        | CharList                      |
/// | [u8] or Vec<u8>                       | vec![1u8, 2u8, 3u8]               | '123'                         | ByteList                      |
/// | Option::Some<T>                       | Some(10)                          | 10                            | Any                           |
/// | Option::None                          | None                              | ()                            | Unit                          |
/// | Unit                                  | Unit                              | ()                            | Unit                          |
/// | Unit Struct                           | struct Unit                       | ()                            | Unit                          |
/// | Unit Variant                          | enum E { A, B }                   | ;E::A                         | Symbol                        |
/// | Newtype Struct                        | struct Seconds(u8)                | 10                            | Any - underlying value        |
/// | Newtype Variant                       | enum E { N(u8) }                  | ;E::N, 10                     | List (enum name, value        |
/// | Sequence                              | vec![1, 2, 3]                     | 1, 2, 3                       | List                          |
/// | Tuple                                 | (1, 2, 3)                         | 1, 2, 3                       | List                          |
/// | Tuple Struct                          | struct RGB(u8, u8, u8)            | 1, 2, 3                       | List                          |
/// | Tuple Variant                         | enum E { T(u8, u8) }              | ;E::T, (1, 2)                 | List                          |
/// | Map                                   | HashMap<K, V>                     | ;one = 1, ;two = 2            | List - with associations      |
/// | Struct                                | struct S { one: u8, two: u8 }     | ;one = 1, ;two = 2            | List - with associations      |
/// | Struct Variant                        | enum E { S { one: u8, two: u8 }   | ;E::S, (;one = 1, ;two = 2)   | List (enum name, struct list) |
///
/// ## Deserialization Type Conversions
///
/// | *Garnish Value*   | *Garnish Type*    | *Compatible Rust Types*                                               |
/// | ()                | Unit              | Unit, None                                                            |
/// | $?                | True              | bool                                                                  |
/// | $!                | False             | bool                                                                  |
/// | 5                 | Number            | i8, i16, i32, i64, u8, u16, u32, u64                                  |
/// | 5.0               | Number            | f32, f64                                                              |
/// | "a"               | Char              | char                                                                  |
/// | "abcd"            | CharList          | String                                                                |
/// | '1'               | Byte              | u8                                                                    |
/// | '1234'            | ByteList          | Vec<u8>                                                               |
/// | #5                | Type              | Enum                                                                  |
/// | ;symbol           | Symbol            | Enum/Unit Variant                                                     |
/// | 5 = 10            | Pair              | ?                                                                     |
/// | 5..10             | Range             | ?                                                                     |
/// | 5 <> 10           | Concatenation     | Vec<T>, String, Map, Struct, Tuple                                    |
/// | list ~ 1..3       | Slice             | Vec<T>, String, Map, Struct, Tuple                                    |
/// | 10, 20, 30        | List              | Vec<T>, Map, Struct, Tuple, Newtype/Tuple/Struct Variant              |
/// | { 5 + 5 }         | Expression        | ?                                                                     |
/// | external_value    | External          | ?                                                                     |
/// | custom_type       | Custom            | ?                                                                     |
///


mod deserializer;
mod deserialize;
mod serializer;
mod serialize;
mod visitor;
mod error;
mod traits;
mod options;

pub use options::*;
pub use traits::*;
pub use serializer::*;
pub use deserializer::GarnishDataDeserializer;
pub use error::GarnishSerializationError;

#[cfg(test)]
mod tests {

}
