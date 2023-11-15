use crate::range_reduction::range_reduce_even;


pub fn cl2(x: f64) -> f64 {
    let (r, sgn) = range_reduce_even(x);

    if r == 0.0 || r == std::f64::consts::PI {
        0.0
    } else {
        if r < 0.5*std::f64::consts::PI {
            let cp = (
                1.3888888888888889e-02, -4.3286930203743071e-04,
                3.2779814789973427e-06, -3.6001540369575084e-09
            );
            let cq = (
                1.0000000000000000e+00, -3.6166589746694121e-02,
                3.6015827281202639e-04, -8.3646182842184428e-07
            );
            let y = r*r;
            let y2 = y*y;
            let p = cp.0 + y*cp.1 + y2*(cp.2 + y*cp.3);
            let q = cq.0 + y*cq.1 + y2*(cq.2 + y*cq.3);

            sgn*r*(1.0 - r.ln() + y*p/q)
        } else {
            let cp = (
                6.4005702446195512e-01, -2.0641655351338783e-01,
                2.4175305223497718e-02, -1.2355955287855728e-03,
                2.5649833551291124e-05, -1.4783829128773320e-07
            );
            let cq = (
                1.0000000000000000e+00, -2.5299102015666356e-01,
                2.2148751048467057e-02, -7.8183920462457496e-04,
                9.5432542196310670e-06, -1.8184302880448247e-08
            );
            let y = std::f64::consts::PI - r;
            let z = y*y - 0.125*std::f64::consts::PI*std::f64::consts::PI;
            let z2 = z*z;
            let z4 = z2*z2;
            let p = cp.0 + z*cp.1 + z2*(cp.2 + z*cp.3) +
                z4*(cp.4 + z*cp.5);
            let q = cq.0 + z*cq.1 + z2*(cq.2 + z*cq.3) +
                z4*(cq.4 + z*cq.5);

            sgn*y*p/q
        }
    }
}
