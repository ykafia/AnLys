use super::*;
use std::fmt;
use std::ops::{Index,IndexMut};
use std::slice::Iter;


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
    pub fn iter(&self) -> Iter<'_, String>  {
        self.values.iter()
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

impl Index<usize> for StringSeries {
    type Output = String;
    fn index(&self, index : usize) -> &String {
        &self.values[index]
    }
}
// impl IndexMut<usize> for StringSeries {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         &mut self.values[index]
//     }
// }

// impl IntoIterator for StringSeries {
//     type Item = String;
//     type IntoIter = StringSeriesIterator;

//     fn into_iter(self) -> StringSeriesIterator {
//         StringSeriesIterator {
//             series : self,
//             index : 0
//         }
//     }
// }

// struct StringSeriesIterator {
//     series : StringSeries,
//     index : usize
// }

// impl Iterator for StringSeriesIterator {
//     type Item = String;
//     fn next(&mut self) -> Option<String> {
//         match     
//     }
// }