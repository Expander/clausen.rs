use crate::range_reduction::range_reduce_odd;


/// Standard Clausen function Cl_5(x) for a real argument
pub fn cl5(x: f64) -> f64 {
    let zeta5 = 1.0369277551433699;
    let r = range_reduce_odd(x);

    if r == 0.0 {
        zeta5
    } else if r < 0.5*std::f64::consts::PI {
        let cp = [
            1.0369277551433699e+00, -6.1354800479984468e-01,
            9.4076401395712763e-02, -9.4056155866704436e-04
        ];
        let cq = [
            1.0000000000000000e+00, -1.2073698633244778e-02,
            1.3703409625482991e-05, -1.9701280330628469e-09,
            2.1944550184416500e-11
        ];
        let y = r*r;
        let y2 = y*y;
        let p = cp[0] + y * cp[1] + y2 * (cp[2] + y * cp[3]);
        let q = cq[0] + y * cq[1] + y2 * (cq[2] + y * cq[3] + y2 * cq[4]);

        p/q - 1.0/24.0*y2*r.ln()
    } else {
        let cp = [
            -4.5930112735784898e-01, 4.3720705508867954e-01,
            -7.5895226486465095e-02, 5.2244176912488065e-03,
            -1.5677716622013956e-04, 1.6641624171748576e-06
        ];
        let cq = [
            1.0000000000000000e+00, -1.2211486825401188e-01,
            3.8940070749313620e-03, -2.2674805547074318e-05,
            -7.4383354448335299e-08, -3.4131758392216437e-10
        ];
        let y = core::f64::consts::PI - r;
        let z = y*y - 0.125*std::f64::consts::PI*std::f64::consts::PI;
        let z2 = z*z;
        let z4 = z2*z2;
        let p = cp[0] + z * cp[1] + z2 * (cp[2] + z * cp[3]) +
            z4 * (cp[4] + z * cp[5]);
        let q = cq[0] + z * cq[1] + z2 * (cq[2] + z * cq[3]) +
            z4 * (cq[4] + z * cq[5]);

        p/q
    }
}
