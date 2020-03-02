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
    pub fn len(&self) -> usize {
        self.values.len()
    }
    pub fn add_empty(&self, maxsize : usize) -> StringSeries {
        let mut result = self.values.clone();
        while maxsize > result.len() {
            println!("pushing some string", );
            result.push("".to_string());
        }
        StringSeries::from_vec(result)
    }
}

impl fmt::Display for StringSeries {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = "[".to_string();
        for element in self.values.iter() {
            result += &format!("\"{}\";", element);
        }
        result += &format!("]");
        write!(f,"{}",result)
    }
}

