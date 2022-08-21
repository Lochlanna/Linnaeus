use std::collections::BTreeMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum PrimitiveValue {
    Bool(bool),
    U8(u8),
    U32(u32),
    U64(u64),
    U128(u128),
    Usize(usize),
    I8(i8),
    I32(i32),
    I64(i64),
    I128(i128),
    Isize(isize),
    F32(f32),
    F64(f64),
    Str(String),
    Char(char),
    Struct(BTreeMap<String, PrimitiveValue>)
}

impl From<bool> for PrimitiveValue {
    fn from(i: bool) -> Self {
        Self::Bool(i)
    }
}

impl<T> From<T> for PrimitiveValue where T: ToBTree{
    fn from(i: T) -> Self {
        Self::Struct(i.to_b_tree())
    }
}

impl From<u8> for PrimitiveValue {
    fn from(i: u8) -> Self {
        Self::U8(i)
    }
}

impl From<u32> for PrimitiveValue {
    fn from(i: u32) -> Self {
        Self::U32(i)
    }
}

impl From<u64> for PrimitiveValue {
    fn from(i: u64) -> Self {
        Self::U64(i)
    }
}

impl From<u128> for PrimitiveValue {
    fn from(i: u128) -> Self {
        Self::U128(i)
    }
}

impl From<usize> for PrimitiveValue {
    fn from(i: usize) -> Self {
        Self::Usize(i)
    }
}

impl From<i8> for PrimitiveValue {
    fn from(i: i8) -> Self {
        Self::I8(i)
    }
}

impl From<i32> for PrimitiveValue {
    fn from(i: i32) -> Self {
        Self::I32(i)
    }
}

impl From<i64> for PrimitiveValue {
    fn from(i: i64) -> Self {
        Self::I64(i)
    }
}

impl From<i128> for PrimitiveValue {
    fn from(i: i128) -> Self {
        Self::I128(i)
    }
}

impl From<isize> for PrimitiveValue {
    fn from(i: isize) -> Self {
        Self::Isize(i)
    }
}

impl From<f32> for PrimitiveValue {
    fn from(i: f32) -> Self {
        Self::F32(i)
    }
}

impl From<f64> for PrimitiveValue {
    fn from(i: f64) -> Self {
        Self::F64(i)
    }
}

impl From<String> for PrimitiveValue {
    fn from(i: String) -> Self {
        Self::Str(i)
    }
}

impl From<char> for PrimitiveValue {
    fn from(i: char) -> Self {
        Self::Char(i)
    }
}

pub trait ToBTree {
    fn to_b_tree(&self) -> std::collections::BTreeMap<String, PrimitiveValue>;
}