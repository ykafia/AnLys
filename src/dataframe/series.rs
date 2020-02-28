use super::*;
use ndarray::LinalgScalar;
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
    
}