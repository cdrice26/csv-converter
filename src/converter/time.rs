pub fn ns_to_s(ns: f64) -> f64 {
    ns / 1_000_000_000.0
}

pub fn s_to_ns(s: f64) -> f64 {
    s * 1_000_000_000.0
}

pub fn us_to_s(us: f64) -> f64 {
    us / 1_000_000.0
}

pub fn s_to_us(s: f64) -> f64 {
    s * 1_000_000.0
}

pub fn ms_to_s(ms: f64) -> f64 {
    ms / 1_000.0
}

pub fn s_to_ms(s: f64) -> f64 {
    s * 1_000.0
}

pub fn min_to_s(min: f64) -> f64 {
    min * 60.0
}

pub fn s_to_min(s: f64) -> f64 {
    s / 60.0
}

pub fn h_to_s(h: f64) -> f64 {
    h * 3600.0
}

pub fn s_to_h(s: f64) -> f64 {
    s / 3600.0
}

pub fn d_to_s(d: f64) -> f64 {
    d * 86_400.0
}

pub fn s_to_d(s: f64) -> f64 {
    s / 86_400.0
}

pub fn w_to_s(w: f64) -> f64 {
    w * 604_800.0
}

pub fn s_to_w(s: f64) -> f64 {
    s / 604_800.0
}

pub fn mo_to_s(mo: f64) -> f64 {
    mo * 2_592_000.0
}

pub fn s_to_mo(s: f64) -> f64 {
    s / 2_592_000.0
}

pub fn y_to_s(y: f64) -> f64 {
    y * 31_536_000.0
}

pub fn s_to_y(s: f64) -> f64 {
    s / 31_536_000.0
}
