use num::complex::Complex;
use clausen::Cl;
mod common;


#[test]
fn test_values() {
    let eps = 1e-14;

    for n in [-10, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 1000, 1001, 1000000].iter() {
        let values = common::read_data_file(&format!("Cl{}.txt", *n)).unwrap();

        for &(v, expected) in values.iter() {
            assert_eq_float!(v.cl(*n), expected, eps);
            assert_eq_float!(Complex::new(v, 0.0).cl(*n).re, expected, eps);
        }
    }
}


#[test]
fn test_cl1() {
    assert_eq!(0.0.cl(1), std::f64::INFINITY);
}
