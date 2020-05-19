
#[derive(Copy, Clone)]
pub struct BitSet(u128);

impl BitSet {
    pub fn get(&self, idx : usize) -> bool {
        1 << idx & self.0 == 1
    }
}

#[derive(Clone)]
pub struct BitSetChecker {
    pub not_null : Vec<BitSet>,
    pub size : usize
}

impl BitSetChecker {
    pub fn get(&self, idx : usize) -> Result<bool, &str> {
        if idx > self.size {
            Err("Index out of range")
        } else {
            Ok(self.not_null[(idx/128)].get(idx%128))
        }
    } 
}

impl IntoIterator for BitSetChecker {
    type Item = bool;
    type IntoIter = BitSetCheckerIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        BitSetCheckerIntoIterator {
            pixel: self,
            index: 0,
        }
    }
}

pub struct BitSetCheckerIntoIterator {
    pixel: BitSetChecker,
    index: usize,
}

impl Iterator for BitSetCheckerIntoIterator {
    type Item = bool;
    fn next(&mut self) -> Option<bool> {
        let result = false;
        self.index += 1;
        Some(result)
    }
}

