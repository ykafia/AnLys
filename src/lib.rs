#[allow(dead_code)]
mod dataframe;
mod prelude;
#[allow(dead_code)]
mod backend;



#[cfg(test)]
mod tests {
    // use super::prelude::*;
    
    use super::backend::*;
    #[test]
    fn data_store_access() {
        let mut x = DataStore::new();
        
        x.insert_num_array("some nums", NumArrayTypes::from(&[0u8,5u8,9u8][..]));
        x.insert_num_array("some floats", NumArrayTypes::from(&[3f32,3.0,4.0,8.5][..]));
        x.insert_str_array("some text",&["hello","world","how","are","you"][..]);
    }

    
}






// TODO: Find a way to fill a dataframe for a csv
// TODO: Find a way to do aggregations
// TODO: Find a way to query on the data
// TODO: Find a way to do a groupby
// TODO: Find a way to do a join between 2 dataframes