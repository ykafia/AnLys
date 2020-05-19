use super::*;



pub enum GenericNumArray {
    F32(F32Array),
    F64(F64Array),
    I32(I32Array),
    I64(I64Array),
}

impl GenericNumArray {
    pub fn try_f32(&self) -> Result<F32Array, &str> {
        match self {
            GenericNumArray::F32(ref a) => Ok(a.clone()),
            _ => Err("Wrong type")
        }
    }
    pub fn try_f64(&self) -> Result<F64Array, &str> {
        match self {
            GenericNumArray::F64(ref a) => Ok(a.clone()),
            _ => Err("Wrong type")
        }
    }
    pub fn try_i32(&self) -> Result<I32Array, &str> {
        match self {
            GenericNumArray::I32(ref a) => Ok(a.clone()),
            _ => Err("Wrong type")
        }
    }
    pub fn try_i64(&self) -> Result<I64Array, &str> {
        match self {
            GenericNumArray::I64(ref a) => Ok(a.clone()),
            _ => Err("Wrong type")
        }
    }
    pub fn len(&self) -> usize {
        match self {
            GenericNumArray::I32(ref a) => a.len(),
            GenericNumArray::F32(ref a) => a.len(),
            GenericNumArray::I64(ref a) => a.len(),
            GenericNumArray::F64(ref a) => a.len(),
        }
    }
    pub fn expand(&mut self, size : usize) {
        match *self {
            GenericNumArray::I32(ref mut a) => a.add_empty(size),
            GenericNumArray::F32(ref mut a) => a.add_empty(size),
            GenericNumArray::I64(ref mut a) => a.add_empty(size),
            GenericNumArray::F64(ref mut a) => a.add_empty(size),
        }
    }
    
}
