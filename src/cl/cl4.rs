use crate::range_reduction::range_reduce_even;


/// Standard Clausen function Cl_4(x) for a real argument
pub fn cl4(x: f64) -> f64 {
    let zeta3 = 1.2020569031595943;
    let (r, sgn) = range_reduce_even(x);

    if r == 0.0 {
        r
    } else if r == std::f64::consts::PI {
        0.0
    } else if r < 0.5*std::f64::consts::PI {
        let cp = [
            -3.0555555555555556e-01,  6.0521392328447206e-03,
            -1.9587493942041528e-05, -3.1137343767030358e-08
        ];
        let cq = [
            1.0000000000000000e+00, -2.2079728398400851e-02,
            1.0887447112236682e-04, -6.1847621370547954e-08
        ];
        let y = r*r;
        let y2 = y*y;
        let p = cp[0] + y * cp[1] + y2 * (cp[2] + y * cp[3]);
        let q = cq[0] + y * cq[1] + y2 * (cq[2] + y * cq[3]);

        sgn*r*(zeta3 + y*(p/q + 1.0/6.0*r.ln()))
    } else {
        let cp = [
            7.6223911686491336e-01, -2.4339587368267260e-01,
            2.8715364937979943e-02, -1.5368612510964667e-03,
            3.6261044225761673e-05, -2.8557977333851308e-07
        ];
        let cq = [
            1.0000000000000000e+00, -1.7465715261403233e-01,
            9.5439417991615653e-03, -1.7325070821666274e-04,
            5.9283675098376635e-07,  9.4127575773361230e-10
        ];
        let y = std::f64::consts::PI - r;
        let z = y*y - 0.125*std::f64::consts::PI*std::f64::consts::PI;
        let z2 = z*z;
        let z4 = z2*z2;
        let p = cp[0] + z * cp[1] + z2 * (cp[2] + z * cp[3]) +
            z4 * (cp[4] + z * cp[5]);
        let q = cq[0] + z * cq[1] + z2 * (cq[2] + z * cq[3]) +
            z4 * (cq[4] + z * cq[5]);

        sgn*y*p/q
    }
}
