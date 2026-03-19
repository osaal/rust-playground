use std::ops::Add;

pub struct Base(isize);
pub struct Scientific {
    pub discriminand: f64,
    exponent: Base,
}

impl Scientific {
    pub fn new(d: f64, e: isize) -> Self {
        Self {
            discriminand: d,
            exponent: Base(e),
        }
    }
}

impl From<(f64, isize)> for Scientific {
    fn from(value: (f64, isize)) -> Self {
        Self {
            discriminand: value.0,
            exponent: Base(value.1),
        }
    }
}
