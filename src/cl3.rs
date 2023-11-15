use crate::range_reduction::range_reduce_odd;


/// Standard Clausen function Cl_3(x) for a real argument
pub fn cl3(x: f64) -> f64 {
    let zeta3 = 1.2020569031595943;
    let r = range_reduce_odd(x);

    if r == 0.0 {
        zeta3
    } else if r < 0.5*std::f64::consts::PI {
        let cp = (
            -7.5000000000000001e-01,  1.5707637881835541e-02,
            -3.5426736843494423e-05, -2.4408931585123682e-07
        );
        let cq = (
            1.0000000000000000e+00, -2.5573146805410089e-02,
            1.5019774853075050e-04, -1.0648552418111624e-07
        );
        let y = r*r;
        let y2 = y*y;
        let p = cp.0 + y * cp.1 + y2 * (cp.2 + y * cp.3);
        let q = cq.0 + y * cq.1 + y2 * (cq.2 + y * cq.3);

        zeta3 + y*(p/q + 0.5*r.ln())
    } else {
        let cp = (
            -4.9017024647634973e-01, 4.1559155224660940e-01,
            -7.9425531417806701e-02, 5.9420152260602943e-03,
            -1.8302227163540190e-04, 1.8027408929418533e-06
        );
        let cq = (
            1.0000000000000000e+00, -1.9495887541644712e-01,
            1.2059410236484074e-02, -2.5235889467301620e-04,
            1.0199322763377861e-06,  1.9612106499469264e-09
        );
        let y = core::f64::consts::PI - r;
        let z = y*y - 0.125*std::f64::consts::PI*std::f64::consts::PI;
        let z2 = z*z;
        let z4 = z2*z2;
        let p = cp.0 + z * cp.1 + z2 * (cp.2 + z * cp.3) +
            z4 * (cp.4 + z * cp.5);
        let q = cq.0 + z * cq.1 + z2 * (cq.2 + z * cq.3) +
            z4 * (cq.4 + z * cq.5);

        p/q
    }
}
