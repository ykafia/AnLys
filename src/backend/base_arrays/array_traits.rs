use super::*;
use num::{Float,Integer};

// pub trait ToGenerics {
//     fn to_generic(&self) -> GenericNumArray;
// }

// impl ToGenerics for I32Array{
//     fn to_generic(&self) -> GenericNumArray {
//         GenericNumArray::I32(self.clone())
//     }
// }
// impl ToGenerics for F32Array{
//     fn to_generic(&self) -> GenericNumArray {
//         GenericNumArray::F32(self.clone())
//     }
// }
// impl ToGenerics for I64Array{
//     fn to_generic(&self) -> GenericNumArray {
//         GenericNumArray::I64(self.clone())
//     }
// }
// impl ToGenerics for F64Array{
//     fn to_generic(&self) -> GenericNumArray {
//         GenericNumArray::F64(self.clone())
//     }
// }

pub trait ArrayData {
    fn get_mask(&self) -> &BitSetChecker;
    fn get_mut_mask(&mut self) -> &BitSetChecker;
    fn len(&self) -> usize;
}

impl<T : Float + Clone> ArrayData for FloatArray<T> {
    fn get_mask(&self) -> &BitSetChecker {
        &self.mask
    }
    fn get_mut_mask(&mut self) -> &BitSetChecker {
        &self.mask
    }
    fn len(&self) -> usize {
        self.mask.size
    }
    
}
impl<T : Integer + Clone> ArrayData for IntArray<T> {
    fn get_mask(&self) -> &BitSetChecker{
        &self.mask
    }
    fn get_mut_mask(&mut self) -> &BitSetChecker {
       &self.mask
    }
    fn len(&self) -> usize {
        self.mask.size
    }
}

pub trait ExpandArray {
    fn add_empty(&mut self, empty_slots : usize);
}

impl ExpandArray for F32Array {
    fn add_empty(&mut self, empty_slots : usize) {
        self.array = {
            let mut temp = self.array.to_vec();
            for _ in 0..empty_slots {
                temp.push(0.0);
            }
            Array1::from(temp)
        }
    }
}
impl ExpandArray for F64Array {
    fn add_empty(&mut self, empty_slots : usize) {
        self.array = {
            let mut temp = self.array.to_vec();
            for _ in 0..empty_slots {
                temp.push(0.0);
            }
            Array1::from(temp)
        }
    }
}
impl ExpandArray for I32Array {
    fn add_empty(&mut self, empty_slots : usize) {
        self.array = {
            let mut temp = self.array.to_vec();
            for _ in 0..empty_slots {
                temp.push(0);
            }
            Array1::from(temp)
        }
    }
}
impl ExpandArray for I64Array {
    fn add_empty(&mut self, empty_slots : usize) {
        self.array = {
            let mut temp = self.array.to_vec();
            for _ in 0..empty_slots {
                temp.push(0);
            }
            Array1::from(temp)
        }
    }
}



