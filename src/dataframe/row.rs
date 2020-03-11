use super::*;
use std::collections::HashMap;

pub struct Row<T> 
    where T : DTypeName
{
    pub values : HashMap<String,T>
}
