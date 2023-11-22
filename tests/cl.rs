use num::complex::Complex;
use num::Float;
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
fn test_complex_input() {
    let eps = 1e-14;

    assert_eq_complex!(Complex::new(1.0,  1.0).cl(-2), Complex::new( 0.50688457710655124,  0.50950616274374456), eps);
    assert_eq_complex!(Complex::new(1.0, -1.0).cl(-2), Complex::new( 0.50688457710655124, -0.50950616274374456), eps);
    assert_eq_complex!(Complex::new(1.0,  1.0).cl(-1), Complex::new(-0.08267495282794197,  0.49171277760817954), eps);
    assert_eq_complex!(Complex::new(1.0, -1.0).cl(-1), Complex::new(-0.08267495282794197, -0.49171277760817954), eps);
    // assert_eq_complex!(Complex::new(1.0,  1.0).cl( 0), Complex::new(0.5, 0.5).cot()/2.0, eps);
    // assert_eq_complex!(Complex::new(1.0, -1.0).cl( 0), Complex::new(0.5, 0.5).coth()*Complex::<f64>::i()/2.0, eps);
    assert_eq_complex!(Complex::new(0.0,  1.0).cl( 1), Complex::new(-0.0413248546129181, -1.5707963267948966), eps);
    assert_eq_complex!(Complex::new(0.0, -1.0).cl( 1), Complex::new(-0.0413248546129181, -1.5707963267948966), eps);
    assert_eq_complex!(Complex::new(0.0,  1.0).cl( 2), Complex::new( 1.5707963267948966,  0.9861797794993302), eps);
    assert_eq_complex!(Complex::new(0.0, -1.0).cl( 2), Complex::new(-1.5707963267948966, -0.9861797794993302), eps);
    assert_eq_complex!(Complex::new(1.0,  1.0).cl( 2), Complex::new( 1.4107754938116412, -0.1044778629291566), eps);
    assert_eq_complex!(Complex::new(1.0, -1.0).cl( 2), Complex::new( 1.4107754938116412,  0.1044778629291566), eps);
}


#[test]
fn test_cl1() {
    assert_eq!(0.0.cl(1), std::f64::INFINITY);
}
