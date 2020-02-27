use ndarray::ScalarOperand;
use ndarray::LinalgScalar;
use super::*;
use ndarray::prelude::*;
use ndarray::{CowArray, ArrayView1,ArcArray};
use std::ops::*;

type ValueArray<T> = Array1<T>;

pub struct Series<T>
where
    T: LinalgScalar + DTypeName
{
    values: ValueArray<T>,
    dtype: DType,
    len: usize,
}

impl<T> Series<T>
where
    T: LinalgScalar + DTypeName,
{
    pub fn from_array(array: &Array1<T>) -> Result<Series<T>, &'static str> {
        if array.shape()[0] == 0 {
            Err("Array is empty")
        }
        else {
            Ok(Series {
                values: array.clone(),
                dtype: array[0].get_dtype(),
                len: array.dim(),
            })
        }
        
    }
}
/// Operators with series 
impl<T> Add for Series<T>
where
    T: LinalgScalar + DTypeName
{
    type Output = Self;
    fn add(self, other: Series<T>) -> Series<T> {
        Series::from_array(&(&self.values + &other.values)).expect("addition not succeeded")
    }
}
impl<T> Sub for Series<T>
where
    T: LinalgScalar + DTypeName
{
    type Output = Self;
    fn sub(self, other: Series<T>) -> Series<T> {
        Series::from_array(&(&self.values - &other.values)).expect("addition not succeeded")
    }
}
impl<T> Mul for Series<T>
where
    T: LinalgScalar + DTypeName
{
    type Output = Self;
    fn mul(self, other: Series<T>) -> Series<T> {
        Series::from_array(&(&self.values * &other.values)).expect("addition not succeeded")
    }
}
impl<T> Div for Series<T>
where
    T: LinalgScalar + DTypeName
{
    type Output = Self;
    fn div(self, other: Series<T>) -> Series<T> {
        Series::from_array(&(&self.values / &other.values)).expect("addition not succeeded")
    }
}

/// With scalar
/// 
/// 

impl<T> Add<T> for Series<T>
where
    T: ScalarOperand + LinalgScalar + DTypeName 
{
    type Output = Self;
    fn add(self, scalar : T) -> Series<T> {
        Series::from_array(&(&self.values+scalar)).expect("addition not succeeded")
    }
}
impl<T> Sub<T> for Series<T>
where
    T: ScalarOperand + LinalgScalar + DTypeName 
{
    type Output = Self;
    fn sub(self, scalar : T) -> Series<T> {
        Series::from_array(&(&self.values-scalar)).expect("addition not succeeded")
    }
}
impl<T> Mul<T> for Series<T>
where
    T: ScalarOperand + LinalgScalar + DTypeName 
{
    type Output = Self;
    fn mul(self, scalar : T) -> Series<T> {
        Series::from_array(&(&self.values*scalar)).expect("addition not succeeded")
    }
}
impl<T> Div<T> for Series<T>
where
    T: ScalarOperand + LinalgScalar + DTypeName 
{
    type Output = Self;
    fn div(self, scalar : T) -> Series<T> {
        Series::from_array(&(&self.values/scalar)).expect("addition not succeeded")
    }
}

