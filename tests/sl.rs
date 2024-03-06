use num::complex::Complex;
use num::Float;
use clausen::Sl;
mod common;


#[test]
fn test_values() {
    let eps = 1e-14;

    for n in [-2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 1000, 1001].iter() {
        let values = common::read_data_file(&format!("Sl{}.txt", *n)).unwrap();

        for &(v, expected) in values.iter() {
            assert_eq_float!(v.sl(*n), expected, eps);
            assert_eq_float!(Complex::new(v, 0.0).sl(*n).re, expected, eps);
        }
    }
}


#[test]
fn test_complex_input() {
    let eps = 1e-14;

    assert_eq_complex!(Complex::new(0.0,  1.0).sl(1), Complex::new(  1.5707963267948966, -0.5), eps);
    assert_eq_complex!(Complex::new(0.0, -1.0).sl(1), Complex::new( -1.5707963267948966,  0.5), eps);
    assert_eq_complex!(Complex::new(0.0,  1.0).sl(2), Complex::new(  1.3949340668482264, -1.5707963267948966), eps);
    assert_eq_complex!(Complex::new(0.0, -1.0).sl(2), Complex::new(  1.3949340668482264, -1.5707963267948966), eps);
    assert_eq_complex!(Complex::new(1.0,  1.0).sl(2), Complex::new(  0.07413774005332982, -1.07079632679489662), eps);
    assert_eq_complex!(Complex::new(1.0, -1.0).sl(2), Complex::new(  0.07413774005332982,  1.07079632679489662), eps);
    assert_eq_complex!(Complex::new(0.0,  1.0).sl(3), Complex::new(  0.7853981633974483,  1.5616007335148931), eps);
    assert_eq_complex!(Complex::new(0.0, -1.0).sl(3), Complex::new( -0.7853981633974483, -1.5616007335148931), eps);
}


#[test]
fn test_signed_zero() {
    let pz64 = 0.0_f64;
    let nz64 = -0.0_f64;

    for n in (1..101).into_iter().step_by(2) {
        assert!(pz64.sl(n).is_sign_positive());
        assert!(nz64.sl(n).is_sign_negative());

        assert!(Complex::new(pz64, pz64).sl(n).re.is_sign_positive());
        assert!(Complex::new(pz64, pz64).sl(n).im.is_sign_positive());
        assert!(Complex::new(pz64, nz64).sl(n).re.is_sign_positive());
        assert!(Complex::new(pz64, nz64).sl(n).im.is_sign_negative());
        assert!(Complex::new(nz64, pz64).sl(n).re.is_sign_negative());
        assert!(Complex::new(nz64, pz64).sl(n).im.is_sign_positive());
        assert!(Complex::new(nz64, nz64).sl(n).re.is_sign_negative());
        assert!(Complex::new(nz64, nz64).sl(n).im.is_sign_negative());
    }
}
