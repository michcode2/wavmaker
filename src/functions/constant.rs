use crate::function::Function;
pub struct Constant {
    value: f64,
}

impl Function for Constant {
    fn at(&self, _: f64) -> f64 {
        return self.value;
    }
}

impl Constant {
    pub fn new(value: f64) -> Constant {
        Constant { value }
    }
}
