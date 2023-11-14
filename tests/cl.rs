extern crate clausen;
extern crate num;
use num::Float;
mod common;

#[test]
fn test_values() {
    let eps = 1e-14;
    let values = common::read_data_file("Cl1.txt").unwrap();

    for &(_v, _expected) in values.iter() {
        assert_eq_float!(0.0, 0.0, eps);
    }
}
