use super::*;
use std::fmt;
use ndarray::LinalgScalar;
use std::ops::Index;
use std::ops::IndexMut;
use std::str::FromStr;
use std::collections::HashMap;
#[macro_use]
use prettytable::{Table, Row, Cell,Attr, color};

pub struct Frame<T>
    where 
        T : LinalgScalar + DTypeName + FromStr + fmt::Display
{
    pub columns_data : HashMap<String,GenericSeries<T>>
}

impl<T> Frame<T> 
    where 
        T : LinalgScalar + DTypeName + FromStr + fmt::Display
{
    pub fn new() -> Frame<T> {
        Frame {
            columns_data : HashMap::new()
        }
    }
    pub fn add_empty_from_map(&mut self) {
        let max_size : usize = self.columns_data.iter().map(|(_,val)| val.len()).max().unwrap_or(0);
        self.columns_data = self.columns_data
            .iter()
            .map(
                |(key,val)|
                (key.clone(),val.add_empty(max_size))
            )
            .collect::<Vec<(String,GenericSeries<T>)>>()
            .iter()
            .cloned()
            .collect();
    }
    pub fn row_len(&self) -> usize {
        self.columns_data.iter().map(|(_,val)| val.len()).max().unwrap_or(0)
    }
    pub fn add_column(&mut self,series : GenericSeries<T>,name : Option<&str>) {
        self.columns_data.insert(name.unwrap_or("").to_string(), series);
        self.add_empty_from_map();
    }
    pub fn get_column(&mut self, name : &str ) -> &GenericSeries<T> {
        self.columns_data.get(&name.to_string()).expect("something")
    }
    pub fn get_mut_column(&mut self, name : &str ) -> &mut GenericSeries<T> {
        self.columns_data.get_mut(&name.to_string()).expect("something")
    }
    pub fn get_row(&self, idx : usize) -> Vec<(String,String)>{
        let mut result = Vec::new();
        for (key,value) in self.columns_data.iter() {
            result.push((key.clone(),value.get_idx(idx)));
        }
        result
    }
}

impl<T> fmt::Display for Frame<T>
    where T : fmt::Display + DTypeName + LinalgScalar + FromStr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();
        let mut names = vec!["ID".to_string()];
        names.extend(self.columns_data.iter().map(|(k,_)| k.clone()).collect::<Vec<String>>());
        table.add_row(
            Row::new(
                names
                .iter()
                .map(
                    |x|
                    Cell::new(x)
                        .with_style(Attr::Bold)
                        // .with_style(Attr::ForegroundColor(color::GREEN)),
                )
                .collect()
            )
        );
        for i in 0..self.row_len() {
            let mut values = vec![i.to_string()];
            values.extend(self.get_row(i).iter().map(|(_,v)| v.clone()).collect::<Vec<String>>());

            table.add_row(
                Row::new(
                    values
                    .iter()
                    .map(
                        |x|
                        Cell::new(x)
                            // .with_style(Attr::Bold)
                            // .with_style(Attr::ForegroundColor(color::GREEN)),
                    )
                    .collect()
            ));
        }
        write!(f,"{}",table)
    }
}

impl<T> Index<&str> for Frame<T> 
    where T : LinalgScalar + DTypeName + FromStr + fmt::Display
{
    type Output = GenericSeries<T>;
    fn index(&self, index : &str) -> &GenericSeries<T> {
        &self.columns_data[index]
    }
}
