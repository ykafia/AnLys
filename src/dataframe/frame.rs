use super::*;
use std::fmt;
use ndarray::LinalgScalar;


pub struct Frame<T>
    where T : LinalgScalar + DTypeName
{
    pub columns : Vec<GenericSeries<T>>,
    pub column_names : Vec<String>
}

impl<T> Frame<T> 
    where T : LinalgScalar + DTypeName
{
    pub fn new() -> Frame<T> {
        Frame {
            columns : Vec::new(),
            column_names : Vec::new()
        }
    }
    pub fn add_empty(&mut self) {
        let max_size : usize = self.columns.iter().map(|x| x.len()).max().unwrap_or(0);
        self.columns = self.columns.iter().map(|x| x.add_empty(max_size)).collect();
    }
    pub fn add_column(&mut self,series : GenericSeries<T>,name : Option<&str>) {
        self.column_names.push(name.unwrap_or("").to_string());
        self.columns.push(series);
    }
}

impl<T> fmt::Display for Frame<T>
    where T : fmt::Display + DTypeName + LinalgScalar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = "".to_string();
        for i in 0..self.columns.len() {
            result.push_str(&format!("\"{}\" => {}\n",self.column_names[i],self.columns[i]));
        }
        write!(f,"{}",result)
    }
}