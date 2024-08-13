use std::error::Error;

use crate::normalizer;

pub mod length;

pub fn convert(val: f64, from: &str, to: &str) -> Result<f64, Box<dyn Error>> {
    let normalized_from = normalizer::normalize(from)?;
    let converted_to_std = match normalized_from {
        "km" => length::km_to_mm(val),
        "cm" => length::cm_to_mm(val),
        "um" => length::um_to_mm(val),
        "nm" => length::nm_to_mm(val),
        "mi" => length::mi_to_mm(val),
        "yd" => length::yd_to_mm(val),
        "in" => length::in_to_mm(val),
        "ft" => length::ft_to_mm(val),
        "m" => length::m_to_mm(val),
        "mm" => val,
        _ => return Err("from unit not recognized".into()),
    };

    let normalized_to = normalizer::normalize(to)?;
    let converted_to_desired = match normalized_to {
        "km" => length::mm_to_km(converted_to_std),
        "cm" => length::mm_to_cm(converted_to_std),
        "um" => length::mm_to_um(converted_to_std),
        "nm" => length::mm_to_nm(converted_to_std),
        "mi" => length::mm_to_mi(converted_to_std),
        "yd" => length::mm_to_yd(converted_to_std),
        "in" => length::mm_to_in(converted_to_std),
        "ft" => length::mm_to_ft(converted_to_std),
        "m" => length::mm_to_m(converted_to_std),
        "mm" => converted_to_std,
        _ => return Err("to unit not recognized".into()),
    };
    Ok(converted_to_desired)
}
