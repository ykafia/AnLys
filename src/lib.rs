mod dataframe;
mod prelude;

#[cfg(test)]
mod tests {
    use ndarray::prelude::*;
    use super::prelude::*;
    
    #[test]
    fn it_works() {
        let s1 = NumSeries::from_vec(vec![1u8,1u8]).expect("created series");
        let s2 = TextSeries::from_vec(vec!["bonobo","bonobo 2"]).expect("created query");
        println!("{}",s2);
    }
}




// TODO: Find a way to fill a dataframe for a csv
// TODO: Find a way to do aggregations
// TODO: Find a way to query on the data
// TODO: Find a way to do a groupby
// TODO: Find a way to do a join between 2 dataframes