use super::*;
use std::fmt;

pub trait TextValue{}

impl TextValue for String{}
impl TextValue for &str {}

#[derive(Clone)]
pub struct TextSeries<T>
where
    T: TextValue + DTypeName
{
    pub values: Vec<T>,
    pub dtype: DType,
    pub len: usize,
}

impl<T> TextSeries<T>
where
    T: TextValue + DTypeName + Clone,
{
    pub fn from_vec(source : Vec<T>) -> Result<TextSeries<T>, &'static str>{
        if source.len() == 0 {
            Err("Array is empty")
        }
        else  {
            let this_type = source.to_vec()[0].get_dtype();
            Ok(
                TextSeries {
                    values : source.to_vec(),
                    dtype : this_type,
                    len : source.len()
                }
            )
        }
    }
    pub fn len(&self) -> usize {
        self.values.len()
    }
}

impl<T> fmt::Display for TextSeries<T>
    where T : DTypeName + TextValue + fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = "\n[".to_string();
        for element in self.values.iter() {
            result += &format!("\"{}\";", element);
        }
        result += &format!("]\n");
        write!(f,"{}",result)
    }
}

