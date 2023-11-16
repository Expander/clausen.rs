use clausen::Cl;
mod common;

#[test]
fn test_values() {
    let eps = 1e-14;

    for n in [1, 2, 3, 4, 5, 6].iter() {
        let values = common::read_data_file(&format!("Cl{}.txt", *n)).unwrap();

        for &(v, expected) in values.iter() {
            assert_eq_float!(v.cl(*n), expected, eps);
        }
    }
}
