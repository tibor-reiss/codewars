const GRAVITY_ACC: f64 = 9.81 / 1000.0 * 3600.0 * 60.0;  // km/h/min
const MASS: f64 = 80.0; // kg
const D_WATTS: f64 = 0.5; // W/min
const DELTA_T: f64 = 1.0 / 60.0; // min
const DRAG: f64 = 60.0 * 0.3 / 3.6; // kg * h / km / min
const G_THRUST: f64 = 60.0 * 3.6 * 3.6; // 1 / W * kg * km^2 / h^2 / min

fn acceleration(slope: i32, v: f64, watts: f64) -> f64 {
    // Calculate in km/h/min
    let thrust = if v < 0.0 || watts < 0.0 {0.0} else {G_THRUST * watts / (v * MASS)};
    let slope_real = ((slope as f64) / 100.0).atan().sin();
    let a = 
        - GRAVITY_ACC * slope_real
        - DRAG * v.abs() * v.abs() / MASS
        + thrust;
    match a.abs() < 1.0e-5 {
        true => 0.0,
        false => a
    }
}

pub fn temps(v0: i32, slope: i32, d_tot: i32) -> i32 {
    let mut watts = 225.0; // W
    let mut a: f64; // km/h/min
    let mut t = 0.0; // h
    let mut s_left = d_tot as f64;
    let mut v = v0 as f64;  // km/h

    loop {
        t += DELTA_T;
        watts -= D_WATTS * DELTA_T;
        a = acceleration(slope, v as f64, watts);
        v += a * DELTA_T;
        s_left -= v * DELTA_T / 60.0;
        if s_left <= 0.0 {
            break
        }
        if v - 3.0 < 1.0e-2 {
            break
        }
    }
    match s_left > 0.0 {
        true => -1,
        false => t.round() as i32,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(v0: i32, slope: i32, d_tot: i32, exp: i32) -> () {
      assert_eq!(exp, temps(v0, slope, d_tot))
    }

    #[test]
    fn basic_tests() {
      dotest(30, 5, 30, 114);
      dotest(30, 20, 30, -1);
      dotest(30, 8, 20, 110);
      dotest(30, 0, 5, 9);
      dotest(50, 10, 25, 185);
    }
}
