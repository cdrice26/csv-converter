pub fn k_to_c(k: f64) -> f64 {
    k - 273.15
}

pub fn c_to_k(c: f64) -> f64 {
    c + 273.15
}

pub fn f_to_c(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}

pub fn c_to_f(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}
