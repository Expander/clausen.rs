use clausen::Sl;
mod common;


#[test]
#[should_panic]
fn test_non_positive_order() {
    1.0.sl(0);
}


#[test]
fn test_values() {
    let eps = 1e-14;

    for n in [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 1000, 1001].iter() {
        let values = common::read_data_file(&format!("Sl{}.txt", *n)).unwrap();

        for &(v, expected) in values.iter() {
            assert_eq_float!(v.sl(*n), expected, eps);
        }
    }
}
