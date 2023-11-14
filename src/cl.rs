pub trait Cl<T> {
    fn cl(&self, n: i32) -> T;
}

impl Cl<f64> for f64 {
    fn cl(&self, n: i32) -> f64 {
        0.0
    }
}
