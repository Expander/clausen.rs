use clausen::Cl;
mod common;

#[test]
#[should_panic]
fn test_non_positive_order() {
    1.0.cl(0);
}

#[test]
fn test_values() {
    let eps = 1e-14;

    for n in [1, 2, 3, 4, 5, 6, 10, 11, 12, 13, 14, 15, 16, 1000, 1001, 1000000].iter() {
        let values = common::read_data_file(&format!("Cl{}.txt", *n)).unwrap();

        for &(v, expected) in values.iter() {
            assert_eq_float!(v.cl(*n), expected, eps);
        }
    }
}
