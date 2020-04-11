use super::*;
use ndarray::LinalgScalar;
use std::fmt;
use std::marker::PhantomData;
use std::str::FromStr;
use std::borrow::Cow;
#[derive(Clone)]
pub enum GenericSeries<T>
    where 
        T : LinalgScalar + DTypeName + FromStr + fmt::Display 
{
    StringSeries(StringSeries),
    NumSeries(NumSeries<T>),
}

impl<T> GenericSeries<T>
    where 
        T : LinalgScalar + DTypeName + FromStr + fmt::Display 
{
    pub fn try_as_numseries(&self) -> Result<NumSeries<T>,&'static str> {
        match *self {
            GenericSeries::NumSeries(ref n) => Ok(n.clone()),
            _ => Err("It's a num series")
        }
    }
    pub fn try_as_textseries(&self) -> Result<StringSeries,&'static str> {
        match *self {
            GenericSeries::StringSeries(ref s) => Ok(s.clone()),
            _ => Err("It's a num series")
        }
    }
    pub fn len(&self) -> usize {
        match *self {
            GenericSeries::StringSeries(ref s) => s.len(),
            GenericSeries::NumSeries(ref n) => n.len(),
            _ => 0
        }
    }
    pub fn add_empty(&self, max_size : usize) -> GenericSeries<T>{
        match *self {
            GenericSeries::StringSeries(ref s) => 
                GenericSeries::StringSeries(s.add_empty(max_size)),
            GenericSeries::NumSeries(ref n) => match n.add_empty(max_size) {
                Some(x) => GenericSeries::NumSeries(x),
                None => GenericSeries::NumSeries(n.clone())
            },
            _ => panic!("You shouldn't have a series as a phantom data")
        }
    }pub fn get_idx_string(&self, idx : usize) -> String {
        match *self {
            GenericSeries::StringSeries(ref s) => s[idx].clone(),
            GenericSeries::NumSeries(ref n) => n[idx].to_string(),
            _ => panic!("You shouldn't have a Phantom series")
        }
    }

    pub fn get_idx(&self, idx : usize) -> DType {
        match *self {
            GenericSeries::StringSeries(ref s) => {
                DType::StringValue(s[idx].clone())
            },
            GenericSeries::NumSeries(ref n) => {
                match try_into_dtype::<T>(&n[idx]) {
                    Some(r) => r,
                    None => panic!("Bad conversion, my bad")
                }
            },
            _ => panic!("You shouldn't have a Phantom series")
        }
    }
    
}

impl<T> fmt::Display for GenericSeries<T>
    where 
        T : fmt::Display + DTypeName + LinalgScalar + FromStr + fmt::Display 
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        match self {
            GenericSeries::NumSeries(ref n) =>  write!(f,"{}",n),
            GenericSeries::StringSeries(ref s) =>  write!(f,"{}",s),
            _ => panic!("You shouldn't have a Phantom series")
        }
    }
}


