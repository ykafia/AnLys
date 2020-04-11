use super::*;
use std::collections::HashMap;
use std::any::*;

#[derive(Clone,Debug)]
pub struct FrameRow<'b>
{
    pub values : HashMap<&'b str,DType>
}

impl<'b> FrameRow<'b>
{
    pub fn new(values: HashMap<&'b str,DType>) -> Self { Self { values } }
}

