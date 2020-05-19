use super::*;

impl<F : Float + Clone> From<Vec<F>> for FloatArray<F>{
    fn from(values : Vec<F>) -> Self {
        FloatArray {
            array : Array1::<F>::from(values.clone()),
            mask : BitSetChecker {
                size : values.len(),
                //TODO : create a true not_null mask
                not_null : Vec::new()
            }
        }
    }
}
impl<F : Integer + Clone> From<Vec<F>> for IntArray<F>{
    fn from(values : Vec<F>) -> Self {
        IntArray {
            array : Array1::<F>::from(values.clone()),
            mask : BitSetChecker {
                size : values.len(),
                //TODO : create a true not_null mask
                not_null : Vec::new()
            }
        }
    }
}
impl<T : Num + Clone> From<Vec<T>> for NumArray<T>{
    fn from(values : Vec<T>) -> Self {
        NumArray {
            array : Array1::<T>::from(values.clone()),
            mask : BitSetChecker {
                size : values.len(),
                //TODO : create a true not_null mask
                not_null : Vec::new()
            }
        }
    }
}

impl<T : Integer + Clone> From<NumArray<T>> for IntArray<T>{
    fn from(values : NumArray<T>) -> Self {
        IntArray {
            array : Array1::<T>::from(values.array.to_vec()),
            mask : BitSetChecker {
                size : values.array.len(),
                //TODO : create a true not_null mask
                not_null : Vec::new()
            }
        }
    }
}
impl<T : Float + Clone> From<NumArray<T>> for FloatArray<T>{
    fn from(values : NumArray<T>) -> Self {
        FloatArray {
            array : Array1::<T>::from(values.array.to_vec()),
            mask : BitSetChecker {
                size : values.array.len(),
                //TODO : create a true not_null mask
                not_null : Vec::new()
            }
        }
    }
}

impl Into<GenericNumArray> for F32Array {
    fn into(self) -> GenericNumArray {
        GenericNumArray::F32(self)
    }
}
impl Into<GenericNumArray> for I32Array {
    fn into(self) -> GenericNumArray {
        GenericNumArray::I32(self)
    }
}
impl Into<GenericNumArray> for F64Array {
    fn into(self) -> GenericNumArray {
        GenericNumArray::F64(self)
    }
}
impl Into<GenericNumArray> for I64Array {
    fn into(self) -> GenericNumArray {
        GenericNumArray::I64(self)
    }
}