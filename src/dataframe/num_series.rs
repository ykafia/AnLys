use super::*;
use ndarray::prelude::*;
use ndarray::LinalgScalar;
use ndarray::ScalarOperand;
use std::fmt;
use std::ops::*;
use std::str::FromStr;

#[derive(Clone, PartialEq, Debug)]
pub struct NumSeries<T>
where
    T: LinalgScalar + DTypeName,
{
    pub values: Array1<T>,
    pub tmp: ArcArray<T, Ix1>,
    pub len: usize,

}

impl<T> NumSeries<T>
where
    T: LinalgScalar + DTypeName + FromStr + fmt::Display,
{
    pub fn from_vec(source: Vec<T>) -> Result<NumSeries<T>, &'static str> {
        NumSeries::from_array(&Array1::from(source))
    }
    pub fn from_array(array: &Array1<T>) -> Result<NumSeries<T>, &'static str> {
        if array.shape()[0] == 0 {
            Err("Array is empty")
        } else {
            Ok(NumSeries {
                values: array.clone(),
                tmp: array.clone().into_shared(),
                // dtype: array[0].get_dtype(),
                len: array.dim(),
            })
        }
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn add_empty(&self, maxsize: usize) -> Option<NumSeries<T>> {
        if maxsize > self.values.len() {
            let mut newv = self.values.to_vec();
            for _ in 0..(maxsize - self.values.len()) {
                newv.push(T::zero());
            }
            Some(NumSeries::from_array(&Array::from(newv)).expect("added empty cells"))
        } else {
            None
        }
    }
    pub fn to_vec(&self) -> Vec<T> {
        self.tmp.clone().to_vec()
    }
    pub fn iter(&self) -> ndarray::iter::Iter<'_, T,Ix1> {
        self.values.iter()
    }
    pub fn to_generic(&self) -> GenericSeries<T> {
        GenericSeries::<T>::NumSeries(self.clone())
    }
    pub fn get(&self,id : usize) -> T {
        self.tmp[id]
    }
}

impl<T> fmt::Display for NumSeries<T>
where
    T: fmt::Display + DTypeName + LinalgScalar,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = "[".to_string();
        for element in self.values.iter() {
            result += &format!("{};", element);
        }
        result += &format!("]");
        write!(f, "{}", result)
    }
}

impl<T> Index<usize> for NumSeries<T>
where
    T: DTypeName + LinalgScalar,
{
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.values[index]
    }
}
impl<T> IndexMut<usize> for NumSeries<T>
where
    T: DTypeName + LinalgScalar,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.values[index]
    }
}

/// Operators with series
impl<T> Add for NumSeries<T>
where
    T: LinalgScalar + DTypeName + FromStr + fmt::Display,
{
    type Output = Self;
    fn add(self, other: NumSeries<T>) -> NumSeries<T> {
        NumSeries::from_array(&(&self.values + &other.values)).expect("addition not succeeded")
    }
}
impl<T> Sub for NumSeries<T>
where
    T: LinalgScalar + DTypeName + FromStr + fmt::Display,
{
    type Output = Self;
    fn sub(self, other: NumSeries<T>) -> NumSeries<T> {
        NumSeries::from_array(&(&self.values - &other.values)).expect("addition not succeeded")
    }
}
impl<T> Mul for NumSeries<T>
where
    T: LinalgScalar + DTypeName + FromStr + fmt::Display,
{
    type Output = Self;
    fn mul(self, other: NumSeries<T>) -> NumSeries<T> {
        NumSeries::from_array(&(&self.values * &other.values)).expect("addition not succeeded")
    }
}
impl<T> Div for NumSeries<T>
where
    T: LinalgScalar + DTypeName + FromStr + fmt::Display,
{
    type Output = Self;
    fn div(self, other: NumSeries<T>) -> NumSeries<T> {
        NumSeries::from_array(&(&self.values / &other.values)).expect("addition not succeeded")
    }
}

/// With scalar
///
///

impl<T> Add<T> for NumSeries<T>
where
    T: ScalarOperand + LinalgScalar + DTypeName + FromStr + fmt::Display,
{
    type Output = Self;
    fn add(self, scalar: T) -> NumSeries<T> {
        NumSeries::from_array(&(&self.values + scalar)).expect("addition not succeeded")
    }
}
impl<T> Sub<T> for NumSeries<T>
where
    T: ScalarOperand + LinalgScalar + DTypeName + FromStr + fmt::Display,
{
    type Output = Self;
    fn sub(self, scalar: T) -> NumSeries<T> {
        NumSeries::from_array(&(&self.values - scalar)).expect("addition not succeeded")
    }
}
impl<T> Mul<T> for NumSeries<T>
where
    T: ScalarOperand + LinalgScalar + DTypeName + FromStr + fmt::Display,
{
    type Output = Self;
    fn mul(self, scalar: T) -> NumSeries<T> {
        NumSeries::from_array(&(&self.values * scalar)).expect("addition not succeeded")
    }
}
impl<T> Div<T> for NumSeries<T>
where
    T: ScalarOperand + LinalgScalar + DTypeName + FromStr + fmt::Display,
{
    type Output = Self;
    fn div(self, scalar: T) -> NumSeries<T> {
        NumSeries::from_array(&(&self.values / scalar)).expect("addition not succeeded")
    }
}
