use std::error::Error;

use crate::normalizer;

pub mod angle;
pub mod area;
pub mod length;
pub mod mass;
pub mod speed;
pub mod temperature;
pub mod time;
pub mod volume;

pub fn convert(val: f64, from: &str, to: &str) -> Result<f64, Box<dyn Error>> {
    let normalized_from = normalizer::normalize(from)?;
    let converted_to_std = match normalized_from {
        // Length units
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

        // Volume units
        "m3" => volume::m3_to_mm3(val),
        "km3" => volume::km3_to_mm3(val),
        "cm3" => volume::cm3_to_mm3(val),
        "um3" => volume::um3_to_mm3(val),
        "nm3" => volume::nm3_to_mm3(val),
        "l" => volume::l_to_mm3(val),
        "ml" => volume::ml_to_mm3(val),
        "gal" => volume::gal_to_mm3(val),
        "pt" => volume::pt_to_mm3(val),
        "qt" => volume::qt_to_mm3(val),
        "mm3" => val,

        // Area units
        "km2" => area::km2_to_mm2(val),
        "cm2" => area::cm2_to_mm2(val),
        "um2" => area::um2_to_mm2(val),
        "nm2" => area::nm2_to_mm2(val),
        "mi2" => area::mi2_to_mm2(val),
        "yd2" => area::yd2_to_mm2(val),
        "in2" => area::in2_to_mm2(val),
        "ft2" => area::ft2_to_mm2(val),
        "m2" => area::m2_to_mm2(val),
        "mm2" => val,
        "ac" => area::ac_to_mm2(val),
        "ha" => area::ha_to_mm2(val),

        // Mass units
        "t" => mass::t_to_g(val),
        "mg" => mass::mg_to_g(val),
        "kg" => mass::kg_to_g(val),
        "g" => val,
        "lb" => mass::lb_to_g(val / 16.0),
        "oz" => mass::oz_to_g(val / 35.274),
        "t metric" => mass::t_metric_to_g(val),

        // Temperature units
        "f" => temperature::f_to_c(val),
        "c" => val,
        "k" => temperature::k_to_c(val - 273.15),

        // Time units
        "s" => val,
        "ms" => time::ms_to_s(val),
        "us" => time::us_to_s(val),
        "ns" => time::ns_to_s(val),
        "min" => time::min_to_s(val),
        "h" => time::h_to_s(val),
        "d" => time::d_to_s(val),
        "w" => time::w_to_s(val),
        "mo" => time::mo_to_s(val),
        "y" => time::y_to_s(val),

        // Speed units
        "km/h" => speed::kmh_to_ms(val),
        "m/s" => val,
        "mph" => speed::mph_to_ms(val),

        // Angle units
        "deg" => angle::d_to_rad(val),
        "rad" => val,
        "grad" => angle::grad_to_rad(val),

        // Unknown units
        _ => return Err("from unit not recognized".into()),
    };

    let normalized_to = normalizer::normalize(to)?;
    let converted_to_desired = match normalized_to {
        // Length units
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

        // Volume units
        "m3" => volume::mm3_to_m3(converted_to_std),
        "km3" => volume::mm3_to_km3(converted_to_std),
        "cm3" => volume::mm3_to_cm3(converted_to_std),
        "um3" => volume::mm3_to_um3(converted_to_std),
        "nm3" => volume::mm3_to_nm3(converted_to_std),
        "l" => volume::mm3_to_l(converted_to_std),
        "ml" => volume::mm3_to_ml(converted_to_std),
        "gal" => volume::mm3_to_gal(converted_to_std),
        "pt" => volume::mm3_to_pt(converted_to_std),
        "qt" => volume::mm3_to_qt(converted_to_std),

        // Area units
        "km2" => area::mm2_to_km2(converted_to_std),
        "cm2" => area::mm2_to_cm2(converted_to_std),
        "um2" => area::mm2_to_um2(converted_to_std),
        "nm2" => area::mm2_to_nm2(converted_to_std),
        "mi2" => area::mm2_to_mi2(converted_to_std),
        "yd2" => area::mm2_to_yd2(converted_to_std),
        "in2" => area::mm2_to_in2(converted_to_std),
        "ft2" => area::mm2_to_ft2(converted_to_std),
        "m2" => area::mm2_to_m2(converted_to_std),
        "mm2" => converted_to_std,
        "ac" => area::mm2_to_ac(converted_to_std),
        "ha" => area::mm2_to_ha(converted_to_std),

        // Mass units
        "t" => mass::g_to_t(converted_to_std),
        "kg" => mass::g_to_kg(converted_to_std),
        "g" => converted_to_std,
        "mg" => mass::g_to_mg(converted_to_std),
        "lb" => mass::g_to_lb(converted_to_std),
        "oz" => mass::g_to_oz(converted_to_std),
        "t metric" => mass::g_to_t_metric(converted_to_std),

        // Temperature units
        "f" => temperature::c_to_f(converted_to_std),
        "c" => converted_to_std,
        "k" => temperature::c_to_k(converted_to_std - 273.15),

        // Time units
        "s" => converted_to_std,
        "ns" => time::s_to_ns(converted_to_std),
        "ms" => time::s_to_ms(converted_to_std),
        "us" => time::s_to_us(converted_to_std),
        "min" => time::s_to_min(converted_to_std),
        "h" => time::s_to_h(converted_to_std),
        "d" => time::s_to_d(converted_to_std),
        "w" => time::s_to_w(converted_to_std),
        "mo" => time::s_to_mo(converted_to_std),
        "y" => time::s_to_y(converted_to_std),

        // Speed units
        "km/h" => speed::ms_to_kmh(converted_to_std),
        "m/s" => converted_to_std,
        "mph" => speed::ms_to_mph(converted_to_std),

        // Angle units
        "rad" => converted_to_std,
        "deg" => angle::rad_to_d(converted_to_std),
        "grad" => angle::rad_to_grad(converted_to_std),

        // Unknown units
        _ => return Err("to unit not recognized".into()),
    };
    Ok(converted_to_desired)
}
