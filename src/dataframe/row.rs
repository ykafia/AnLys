use super::*;
use std::collections::HashMap;

pub struct Row<'col> {
    indexes : HashMap<&'col str,Option<usize>>
}

impl<'col> Row<'col> {
    pub fn new() -> Self { Self { indexes : HashMap::new() } }
}


pub struct Rows<'a> {
    pub row : HashMap<usize,Row<'a>>
}