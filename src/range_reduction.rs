pub fn range_reduce_odd(mut x: f64) -> f64 {
    if x < 0.0 {
        x = -x;
    }

    if x >= 2.0*std::f64::consts::PI {
        x = x.rem_euclid(2.0*std::f64::consts::PI);
    }

    if x > std::f64::consts::PI {
        let p0 = 6.28125;
        let p1 = 0.0019353071795864769253;
        x = (p0 - x) + p1;
    }

    x
}
