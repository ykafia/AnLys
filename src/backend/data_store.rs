use super::*;
use std::collections::HashMap;
use std::any::Any;
#[derive(Clone)]
pub struct DataStore<'names,'array,'content> 
{
    pub num_store : HashMap<&'names str, NumArrayTypes<'array>>,
    pub text_store : HashMap<&'names str,&'array [&'content str]>,
}

impl<'names,'array,'str_content> DataStore<'names, 'array, 'str_content> {
    pub fn new() -> DataStore<'names,'array,'str_content> {
        DataStore {
            num_store : HashMap::new(),
            text_store : HashMap::new()
        }
    }
    pub fn insert_num_array(&mut self, name : &'names str, array : NumArrayTypes<'array>){
        self.num_store
            .insert(name, array)
            .expect("Array inserted");
    }
    
}



