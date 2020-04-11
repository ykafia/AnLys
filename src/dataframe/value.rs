use ndarray::LinalgScalar;
use std::fmt;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum DTypeN {
    StringValue,
    Int8,
    Int16,
    Int32,
    Int64,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Float32,
    Float64,
}

#[derive(Clone, PartialEq, Debug)]
pub enum DType {
    StringValue(String),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float32(f32),
    Float64(f64),
}

pub trait DTypeName {
    fn get_dtype(&self) -> DTypeN;
}

impl DTypeName for String {
    fn get_dtype(&self) -> DTypeN {
        DTypeN::StringValue
    }
}
impl DTypeName for &str {
    fn get_dtype(&self) -> DTypeN {
        DTypeN::StringValue
    }
}
impl DTypeName for u8 {
    fn get_dtype(&self) -> DTypeN {
        DTypeN::UInt8
    }
}
impl DTypeName for u16 {
    fn get_dtype(&self) -> DTypeN {
        DTypeN::UInt16
    }
}
impl DTypeName for u32 {
    fn get_dtype(&self) -> DTypeN {
        DTypeN::UInt32
    }
}
impl DTypeName for u64 {
    fn get_dtype(&self) -> DTypeN {
        DTypeN::UInt64
    }
}
impl DTypeName for i8 {
    fn get_dtype(&self) -> DTypeN {
        DTypeN::Int8
    }
}
impl DTypeName for i16 {
    fn get_dtype(&self) -> DTypeN {
        DTypeN::Int16
    }
}
impl DTypeName for i32 {
    fn get_dtype(&self) -> DTypeN {
        DTypeN::Int32
    }
}
impl DTypeName for i64 {
    fn get_dtype(&self) -> DTypeN {
        DTypeN::Int64
    }
}
impl DTypeName for f32 {
    fn get_dtype(&self) -> DTypeN {
        DTypeN::Float32
    }
}
impl DTypeName for f64 {
    fn get_dtype(&self) -> DTypeN {
        DTypeN::Float64
    }
}

use std::any::*;
pub fn try_into_dtype<'a, T: LinalgScalar + DTypeName + Any>(value: &T) -> Option<DType> {
    let value_any = value as &dyn Any;

    match value.get_dtype() {
        DTypeN::Int8 => match value_any.downcast_ref::<i8>() {
            Some(v) => Some(DType::Int8(*v)),
            None => None,
        },
        DTypeN::Int16 => match value_any.downcast_ref::<i16>() {
            Some(v) => Some(DType::Int16(*v)),
            None => None,
        },
        DTypeN::Int32 => match value_any.downcast_ref::<i32>() {
            Some(v) => Some(DType::Int32(*v)),
            None => None,
        },
        DTypeN::Int64 => match value_any.downcast_ref::<i64>() {
            Some(v) => Some(DType::Int64(*v)),
            None => None,
        },
        DTypeN::UInt8 => match value_any.downcast_ref::<u8>() {
            Some(v) => Some(DType::UInt8(*v)),
            None => None,
        },
        DTypeN::UInt16 => match value_any.downcast_ref::<u16>() {
            Some(v) => Some(DType::UInt16(*v)),
            None => None,
        },
        DTypeN::UInt32 => match value_any.downcast_ref::<u32>() {
            Some(v) => Some(DType::UInt32(*v)),
            None => None,
        },
        DTypeN::UInt64 => match value_any.downcast_ref::<u64>() {
            Some(v) => Some(DType::UInt64(*v)),
            None => None,
        },
        DTypeN::Float32 => match value_any.downcast_ref::<f32>() {
            Some(v) => Some(DType::Float32(*v)),
            None => None,
        },
        DTypeN::Float64 => match value_any.downcast_ref::<f64>() {
            Some(v) => Some(DType::Float64(*v)),
            None => None,
        },
        DTypeN::StringValue => match value_any.downcast_ref::<String>() {
            Some(v) => Some(DType::StringValue(v.clone())),
            None => None,
        },
    }
}

impl fmt::Display for DType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        let value = match *self {
            DType::StringValue(ref v) => v.clone(),
            DType::Int8(ref v) => v.to_string(),
            DType::Int16(ref v) => v.to_string(),
            DType::Int32(ref v) => v.to_string(),
            DType::Int64(ref v) => v.to_string(),
            DType::UInt8(ref v) => v.to_string(),
            DType::UInt16(ref v) => v.to_string(),
            DType::UInt32(ref v) => v.to_string(),
            DType::UInt64(ref v) => v.to_string(),
            DType::Float32(ref v) => v.to_string(),
            DType::Float64(ref v) => v.to_string(),
        };
        write!(f,"{}",value)
    }
}
