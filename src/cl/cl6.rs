use crate::range_reduction::range_reduce_even;


/// Standard Clausen function Cl_6(x) for a real argument
pub fn cl6(x: f64) -> f64 {
    let (r, sgn) = range_reduce_even(x);

    if r == 0.0 || r == std::f64::consts::PI {
        0.0
    } else if r < 0.5*std::f64::consts::PI {
        let cp = [
            1.0369277551433699e+00, -2.087195444107175e-01,
            2.0652251045312954e-02, -1.383438138256840e-04
        ];
        let cq = [
            1.0000000000000000e+00, -8.0784096827362542e-03,
            5.8074568862993102e-06, -5.1960620033050114e-10
        ];
        let y = r*r;
        let y2 = y*y;
        let p = cp[0] + y * cp[1] + y2 * (cp[2] + y * cp[3]);
        let q = cq[0] + y * cq[1] + y2 * (cq[2] + y * cq[3]);

        sgn*r*(p/q - 1.0/120.0*y2*r.ln())
    } else {
        let cp = [
            7.9544504578027050e-01, -1.9255025309738589e-01,
            1.5805208288846591e-02, -5.4175380521534706e-04,
            6.7577493541009068e-06
        ];
        let cq = [
            1.0000000000000000e+00, -7.0798422394109274e-02,
            7.1744189715634762e-04,  3.9098747334347093e-06,
            3.5669441618295266e-08,  2.5315391843409925e-10
        ];
        let y = std::f64::consts::PI - r;
        let z = y*y - 0.125*std::f64::consts::PI*std::f64::consts::PI;
        let z2 = z*z;
        let z4 = z2*z2;
        let p = cp[0] + z * cp[1] + z2 * (cp[2] + z * cp[3]) +
            z4 * cp[4];
        let q = cq[0] + z * cq[1] + z2 * (cq[2] + z * cq[3]) +
            z4 * (cq[4] + z * cq[5]);

        sgn*y*p/q
    }
}
