use super::*;
use ndarray::LinalgScalar;
use std::fmt;

#[derive(Clone)]
pub enum GenericSeries<U>
    where 
        U : LinalgScalar + DTypeName
{
    StringSeries(StringSeries),
    NumSeries(NumSeries<U>)
}

impl<U> GenericSeries<U>
    where 
        U : LinalgScalar + DTypeName
{
    pub fn try_get_numseries(&self) -> Result<NumSeries<U>,&'static str> {
        match *self {
            GenericSeries::NumSeries(ref n) => Ok(n.clone()),
            _ => Err("It's a num series")
        }
    }
    pub fn try_get_textseries(&self) -> Result<StringSeries,&'static str> {
        match *self {
            GenericSeries::StringSeries(ref s) => Ok(s.clone()),
            _ => Err("It's a num series")
        }
    }
    pub fn len(&self) -> usize {
        match *self {
            GenericSeries::StringSeries(ref s) => s.len(),
            GenericSeries::NumSeries(ref n) => n.len()
        }
    }
    pub fn add_empty(&self, max_size : usize) -> GenericSeries<U>{
        match *self {
            GenericSeries::StringSeries(ref s) => 
                GenericSeries::StringSeries(s.add_empty(max_size)),
            GenericSeries::NumSeries(ref n) => match n.add_empty(max_size) {
                Some(x) => GenericSeries::NumSeries(x),
                None => GenericSeries::NumSeries(n.clone())
            }
        }
    }
    
}

impl<T> fmt::Display for GenericSeries<T>
    where T : fmt::Display + DTypeName + LinalgScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            GenericSeries::NumSeries(ref n) =>  write!(f,"{}",n),
            GenericSeries::StringSeries(ref s) =>  write!(f,"{}",s)
        }
    }
}

