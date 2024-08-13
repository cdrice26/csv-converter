use std::error::Error;

use crate::normalizer;

fn mm_to_cm(mm: f64) -> f64 {
    mm / 10.0
}

pub fn convert(val: f64, from: &str, to: &str) -> Result<f64, Box<dyn Error>> {
    let normalized_from = normalizer::normalize(from)?;
    let normalized_to = normalizer::normalize(to)?;
    Ok(val / 10.0)
}
