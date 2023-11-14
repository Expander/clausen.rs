extern crate clausen;
extern crate num;
use clausen::Cl;
use num::Float;
mod common;

#[test]
fn test_values() {
    let eps = 1e-14;
    let n = 1;
    let values = common::read_data_file(&format!("Cl1{}.txt", n)).unwrap();

    for &(v, _expected) in values.iter() {
        assert_eq_float!(v.cl(n), 0.0, eps);
    }
}
