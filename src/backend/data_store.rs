use super::*;
use std::collections::HashMap;
#[derive(Clone)]
pub struct DataStore<'names, 'array, 'content> {
    pub num_store: HashMap<&'names str, NumArrayTypes<'array>>,
    pub text_store: HashMap<&'names str, &'array [&'content str]>,
}

impl<'names, 'array, 'str_content> DataStore<'names, 'array, 'str_content> {
    pub fn new() -> DataStore<'names, 'array, 'str_content> {
        DataStore {
            num_store: HashMap::new(),
            text_store: HashMap::new(),
        }
    }
    pub fn insert_num_array(
        &mut self,
        name: &'names str,
        array: NumArrayTypes<'array>,
    ) -> Result<(), &'static str> {
        if self.get_column_names().contains(&name.to_string()) {
            Err("column already exists")
        } 
        else {
            self.num_store.insert(name, array).expect("Array inserted");
            Ok(())
        }
    }
    pub fn insert_str_array(
        &mut self,
        name: &'names str,
        array: &'array [&'str_content str],
    ) -> Result<(), &'static str> {
        if self.get_column_names().contains(&name.to_string()) {
            Err("column already exists")
        }
        else {
            self.text_store.insert(name, array).expect("Array inserted");
            Ok(())
        }
        
    }
    pub fn get_column_names(&self) -> Vec<String> {
        let mut names = Vec::new();
        let mut text_names = self
            .text_store
            .iter()
            .map(|(k, _)| k.to_string().clone())
            .collect::<Vec<String>>();
        let mut num_names = self
            .num_store
            .iter()
            .map(|(k, _)| k.to_string().clone())
            .collect::<Vec<String>>();
        names.append(&mut text_names);
        names.append(&mut num_names);
        names
    }
}
