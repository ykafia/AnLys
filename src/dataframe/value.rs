use ndarray::LinalgScalar;


#[derive(Copy,Clone,PartialEq,Debug)]
pub enum DType {
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
pub trait DTypeName {
    fn get_dtype(&self) -> DType;
}

impl DTypeName for String{
    fn get_dtype(&self) -> DType{
        DType::StringValue
    }
}
impl DTypeName for &str{
    fn get_dtype(&self) -> DType{
        DType::StringValue
    }
}
impl DTypeName for u8{
    fn get_dtype(&self) -> DType{
        DType::UInt8
    }
}
impl DTypeName for u16{
    fn get_dtype(&self) -> DType{
        DType::UInt16
    }
}
impl DTypeName for u32{
    fn get_dtype(&self) -> DType{
        DType::UInt32
    }
}
impl DTypeName for u64{
    fn get_dtype(&self) -> DType{
        DType::UInt64
    }
}
impl DTypeName for i8{
    fn get_dtype(&self) -> DType{
        DType::Int8
    }
}
impl DTypeName for i16{
    fn get_dtype(&self) -> DType{
        DType::Int16
    }
}
impl DTypeName for i32{
    fn get_dtype(&self) -> DType{
        DType::Int32
    }
}
impl DTypeName for i64{
    fn get_dtype(&self) -> DType{
        DType::Int64
    }
}
impl DTypeName for f32{
    fn get_dtype(&self) -> DType{
        DType::Float32
    }
}
impl DTypeName for f64{
    fn get_dtype(&self) -> DType{
        DType::Float64
    }
}
