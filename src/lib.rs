mod dataframe;
mod prelude;

#[cfg(test)]
mod tests {
    use ndarray::prelude::*;
    use super::prelude::*;
    use std::str::FromStr;
    
    #[test]
    fn dataframe_displayed() {
        let mut x = Frame::<f64>::new();
        
        let s1 = NumSeries::from_vec(vec![1.0,1.0,3.0]).expect("created series");
        let s2 = StringSeries::from_vec(vec!["bonobo".to_string(),"bonobo 2".to_string()]);

        x.add_column(GenericSeries::<f64>::NumSeries(s1),Some("Numbers"));
        x.add_column(GenericSeries::<f64>::StringSeries(s2),Some("Text"));
        assert!(x.to_string().len()>0);
        

    }

    #[test]
    fn add_series(){
        let mut x = Frame::<f64>::new();
        
        let s1 = NumSeries::from_vec(vec![1.0,1.0,3.0]).expect("created series");
        let s2 = NumSeries::from_vec(vec![1.0,1.0,3.0]).expect("created series");

        assert_eq!(s1+s2,NumSeries::from_vec(vec![2.0,2.0,6.0]).expect("created series"))
    }
}




// TODO: Find a way to fill a dataframe for a csv
// TODO: Find a way to do aggregations
// TODO: Find a way to query on the data
// TODO: Find a way to do a groupby
// TODO: Find a way to do a join between 2 dataframes