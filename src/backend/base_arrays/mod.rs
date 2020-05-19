mod array_traits;
mod array_convert;
mod array_kinds;
mod generic;
pub use self::array_traits::*;
pub use self::array_convert::*;
pub use self::array_kinds::*;
pub use self::generic::*;



use num::{Num,Float,Integer};
use ndarray::Array1;
use std::convert::{From,Into};

use super::*;

