pub fn sl21(x: f64) -> f64 {
    let pi = core::f64::consts::PI;
    let pi2 = pi*pi;
    let pi4 = pi2*pi2;
    let pi6 = pi4*pi2;
    let pi8 = pi4*pi4;
    let pi10 = pi6*pi4;
    let pi12 = pi6*pi6;
    let pi14 = pi8*pi6;
    let pi16 = pi8*pi8;
    let pi18 = pi10*pi8;
    let pi20 = pi10*pi10;
    let x2 = x*x;
    x*(174611.0*pi20/1531329465290625.0 + x2*(-43867.0*pi18/233875772880750.0 + x2*(3617.0*pi16/39076987950000.0 + x2*(-1.0/45972927000.0*pi14 + x2*(691.0*pi12/231703552080000.0 + x2*(-1.0/3734416224000.0*pi10 + x2*(pi8/58845346560000.0 + x2*(-1.0/1235752277760000.0*pi6 + x2*(pi4/32011868528640000.0 + x2*(-1.0/729870602452992000.0*pi2 + (pi/4865804016353280000.0 - 1.0/102181884343418880000.0*x)*x))))))))))
}