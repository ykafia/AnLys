#[cfg(test)]
mod tests {
    use ndarray::prelude::*;
    #[test]
    fn it_works() {
        let x = Array1::<i32>::zeros(5);
        let y = Array1::<i32>::ones(5);
        let z = x+y;
        assert_eq!(Array1::<i32>::ones(5),z );
    }
}


mod dataframe;