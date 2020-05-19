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
        let z  = IntArray::from(vec![3i32,2,3,65,8]);
        let w = FloatArray::from(vec![3f32,3.0,3.4]);
        x.insert_num_array("some numbers", z.into()).expect("added column");
        x.insert_num_array("some floats", w.into()).expect("added column");
    }

    
}






// TODO: Find a way to fill a dataframe for a csv
// TODO: Find a way to do aggregations
// TODO: Find a way to query on the data
// TODO: Find a way to do a groupby
// TODO: Find a way to do a join between 2 dataframes