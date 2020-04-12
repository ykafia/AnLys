use super::*;
use std::collections::HashMap;
use std::fmt;
use prettytable::*;

#[derive(Clone,Debug)]
pub struct FrameRow<'b>
{
    pub values : HashMap<&'b str,DType>
}

impl<'b> FrameRow<'b>
{
    pub fn new(values: HashMap<&'b str,DType>) -> Self { Self { values } }
}

impl<'b> fmt::Display for FrameRow<'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { 
        let mut table = Table::new();
        let values = 
            self.values
                .iter()
                .map(|(_,v)| v.to_string()).collect::<Vec<String>>();
        let names = 
            self.values
                .iter()
                .map(|(k,_)| k.to_string()).collect::<Vec<String>>();
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
            )
        );
        
        write!(f,"{}",table)
     }

}