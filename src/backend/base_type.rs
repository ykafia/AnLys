#[derive(Clone,Copy, PartialEq, Debug)]
pub enum BaseType {
    STR,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
}

pub trait DType{
    fn get_dtype(&self) -> BaseType;
}

impl DType for u8 {
    fn get_dtype(&self) -> BaseType {
        BaseType::U8
    }
}
impl DType for u16 {
    fn get_dtype(&self) -> BaseType {
        BaseType::U16
    }
}
impl DType for u32 {
    fn get_dtype(&self) -> BaseType {
        BaseType::U32
    }
}
impl DType for u64 {
    fn get_dtype(&self) -> BaseType {
        BaseType::U64
    }
}
impl DType for i8 {
    fn get_dtype(&self) -> BaseType {
        BaseType::I8
    }
}
impl DType for i16 {
    fn get_dtype(&self) -> BaseType {
        BaseType::I16
    }
}
impl DType for i32 {
    fn get_dtype(&self) -> BaseType {
        BaseType::I32
    }
}
impl DType for i64 {
    fn get_dtype(&self) -> BaseType {
        BaseType::I64
    }
}
impl DType for f32 {
    fn get_dtype(&self) -> BaseType {
        BaseType::F32
    }
}
impl DType for f64 {
    fn get_dtype(&self) -> BaseType {
        BaseType::F64
    }
}

impl DType for str {
    fn get_dtype(&self) -> BaseType {
        BaseType::STR
    }
}

pub trait DTypeArray{}

impl<'a,T> DTypeArray for &'a [T] where T : DType{}


use std::convert::From;
#[derive(Clone,Copy, PartialEq, Debug)]
pub enum NumArrayTypes<'a> {
    I8(&'a [i8]),
    I16(&'a [i16]),
    I32(&'a [i32]),
    I64(&'a [i64]),
    U8(&'a [u8]),
    U16(&'a [u16]),
    U32(&'a [u32]),
    U64(&'a [u64]),
    F32(&'a [f32]),
    F64(&'a [f64]),
}

impl<'a> From<&'a [u8]> for NumArrayTypes<'a> {
    fn from(value: &'a [u8]) -> Self { 
        NumArrayTypes::<'a>::U8(value)
    }
}
impl<'a> From<&'a [i8]> for NumArrayTypes<'a> {
    fn from(value: &'a [i8]) -> Self { 
        NumArrayTypes::<'a>::I8(value)
    }
}
impl<'a> From<&'a [u16]> for NumArrayTypes<'a> {
    fn from(value: &'a [u16]) -> Self { 
        NumArrayTypes::<'a>::U16(value)
    }
}
impl<'a> From<&'a [i16]> for NumArrayTypes<'a> {
    fn from(value: &'a [i16]) -> Self { 
        NumArrayTypes::<'a>::I16(value)
    }
}
impl<'a> From<&'a [u32]> for NumArrayTypes<'a> {
    fn from(value: &'a [u32]) -> Self { 
        NumArrayTypes::<'a>::U32(value)
    }
}
impl<'a> From<&'a [i32]> for NumArrayTypes<'a> {
    fn from(value: &'a [i32]) -> Self { 
        NumArrayTypes::<'a>::I32(value)
    }
}
impl<'a> From<&'a [f32]> for NumArrayTypes<'a> {
    fn from(value: &'a [f32]) -> Self { 
        NumArrayTypes::<'a>::F32(value)
    }
}
impl<'a> From<&'a [f64]> for NumArrayTypes<'a> {
    fn from(value: &'a [f64]) -> Self { 
        NumArrayTypes::<'a>::F64(value)
    }
}
impl<'a> From<&'a [u64]> for NumArrayTypes<'a> {
    fn from(value: &'a [u64]) -> Self { 
        NumArrayTypes::<'a>::U64(value)
    }
}
impl<'a> From<&'a [i64]> for NumArrayTypes<'a> {
    fn from(value: &'a [i64]) -> Self { 
        NumArrayTypes::<'a>::I64(value)
    }
}
