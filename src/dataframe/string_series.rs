use super::*;
use std::fmt;


#[derive(Clone)]
pub struct StringSeries
{
    pub values: Vec<String>,
    pub dtype: DType,
    pub len: usize,
}

impl StringSeries{
    pub fn from_vec(source : Vec<String>) -> StringSeries {
        StringSeries {
            values : source.to_vec(),
            dtype : DType::StringValue,
            len : source.len()
        }
    }
}

impl fmt::Display for StringSeries {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = "\n[".to_string();
        for element in self.values.iter() {
            result += &format!("\"{}\";", element);
        }
        result += &format!("]\n");
        write!(f,"{}",result)
    }
}

