pub fn d_to_rad(d: f64) -> f64 {
    d * std::f64::consts::PI / 180.0
}

pub fn rad_to_d(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}

pub fn grad_to_rad(grad: f64) -> f64 {
    grad * std::f64::consts::PI / 200.0
}

pub fn rad_to_grad(rad: f64) -> f64 {
    rad * 200.0 / std::f64::consts::PI
}
