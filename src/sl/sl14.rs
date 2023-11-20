pub fn sl14(x: f64) -> f64 {
    let pi = core::f64::consts::PI;
    let pi2 = pi*pi;
    let pi4 = pi2*pi2;
    let pi6 = pi4*pi2;
    let pi8 = pi4*pi4;
    let pi10 = pi6*pi4;
    let pi12 = pi6*pi6;
    let pi14 = pi8*pi6;
    let x2 = x*x;
    2.0*pi14/18243225.0 + x2*(-691.0*pi12/1277025750.0 + x2*(pi10/2245320.0 + x2*(-1.0/6804000.0*pi8 + x2*(pi6/38102400.0 + x2*(-1.0/326592000.0*pi4 + x2*(pi2/2874009600.0 + (-1.0/12454041600.0*pi + 1.0/174356582400.0*x)*x))))))
}
