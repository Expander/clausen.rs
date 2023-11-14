pub trait Cl<T> {
    fn cl(&self, n: i32) -> T;
}

impl Cl<f64> for f64 {
    fn cl(&self, n: i32) -> f64 {
        match n {
            1 => cl1(*self),
            _ => 0.0
        }
    }
}

fn cl1(mut x: f64) -> f64 {
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

    if x == 0.0 {
        std::f64::INFINITY
    } else {
        -(2.0*(0.5*x).sin()).ln()
    }
}
