use super::*;
#[derive(Clone)]
pub struct NumArray<T : Num + Clone> {
    pub array : Array1<T>,
    pub mask : BitSetChecker
}

#[derive(Clone)]
pub struct FloatArray<T : Float + Clone> {
    pub array : Array1<T>,
    pub mask : BitSetChecker
}

#[derive(Clone)]
pub struct IntArray<T : Integer + Clone> {
    pub array : Array1<T>,
    pub mask : BitSetChecker
}
pub type F32Array = FloatArray<f32>;
pub type F64Array = FloatArray<f64>;
pub type I32Array = IntArray<i32>;
pub type I64Array = IntArray<i64>;


impl<T : Float + Clone> FloatArray<T> {
    pub fn len(&self) -> usize {
        self.array.len()
    }
}
impl<T : Integer + Clone> IntArray<T> {
    pub fn len(&self) -> usize {
        self.array.len()
    }
}