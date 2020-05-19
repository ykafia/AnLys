use super::*;
use std::collections::HashMap;

pub struct DataStore<'names> {
    pub num_store: HashMap<&'names str, GenericNumArray>,
    pub text_store: HashMap<&'names str, Vec<String>>,
}

impl<'names> DataStore<'names> {
    pub fn new() -> DataStore<'names> {
        DataStore {
            num_store: HashMap::new(),
            text_store: HashMap::new(),
        }
    }
    pub fn insert_num_array(
        &mut self,
        name: &'names str,
        array: GenericNumArray,
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
        array: Vec<String>,
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
    pub fn normalize_size(&mut self) {
        let max_size : usize = 
            std::cmp::max(
                self.num_store.iter().map(|(_,i)| i.len()).max().unwrap_or(0),
                self.text_store.iter().map(|(_,i)| i.len()).max().unwrap_or(0)
            );
        let mut keys = self.num_store.keys().map(|u| u.to_string()).collect::<Vec<String>>();
        for i in keys {
            let len = self.num_store.get(i.as_str()).unwrap().len();
            if len < max_size {
                self.num_store.get_mut(i.as_str()).unwrap().expand(max_size-len) 
            }
        }
        keys = self.text_store.keys().map(|u| u.to_string()).collect::<Vec<String>>();
        for i in keys {
            let len = self.text_store.get(i.as_str()).unwrap().len();
            if len < max_size {
                for _ in 0..max_size-len {
                    self.text_store.get_mut(i.as_str()).unwrap().push("".to_string())
                }                
            }

        }
    }
}
