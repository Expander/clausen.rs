pub fn sl13(x: f64) -> f64 {
    let pi = core::f64::consts::PI;
    let pi2 = pi*pi;
    let pi4 = pi2*pi2;
    let pi6 = pi4*pi2;
    let pi8 = pi4*pi4;
    let pi10 = pi6*pi4;
    let pi12 = pi6*pi6;
    let x2 = x*x;
    x*(691.0*pi12/638512875.0 + x2*(-1.0/561330.0*pi10 + x2*(pi8/1134000.0 + x2*(-1.0/4762800.0*pi6 + x2*(pi4/32659200.0 + x2*(-1.0/239500800.0*pi2 + (pi/958003200.0 - 1.0/12454041600.0*x)*x))))))
}
