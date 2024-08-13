use std::error::Error;

/// Normalize the given unit to the standard unit abbreviation
pub fn normalize(input_unit: &str) -> Result<&str, Box<dyn Error>> {
    match input_unit.to_lowercase().as_str() {
        // Length units
        "meters" | "meter" | "m" => return Ok("m"),
        "kilometers" | "kilometer" | "km" => return Ok("km"),
        "centimeters" | "centimeter" | "cm" => return Ok("cm"),
        "millimeters" | "millimeter" | "mm" => return Ok("mm"),
        "micrometers" | "micrometer" | "um" => return Ok("um"),
        "nanometers" | "nanometer" | "nm" => return Ok("nm"),
        "miles" | "mile" | "mi" => return Ok("mi"),
        "yards" | "yard" | "yd" => return Ok("yd"),
        "feet" | "foot" | "ft" => return Ok("ft"),
        "inches" | "inch" | "in" => return Ok("in"),

        // Volume units
        "cubic meters" | "cubic meter" | "m3" => return Ok("m3"),
        "cubic kilometers" | "cubic kilometer" | "km3" => return Ok("km3"),
        "cubic centimeters" | "cubic centimeter" | "cm3" => return Ok("cm3"),
        "cubic millimeters" | "cubic millimeter" | "mm3" => return Ok("mm3"),
        "cubic micrometers" | "cubic micrometer" | "um3" => return Ok("um3"),
        "cubic nanometers" | "cubic nanometer" | "nm3" => return Ok("nm3"),
        "liters" | "liter" | "l" => return Ok("l"),
        "milliliters" | "milliliter" | "ml" => return Ok("ml"),
        "gallons" | "gallon" | "gal" => return Ok("gal"),
        "pints" | "pint" | "pt" => return Ok("pt"),
        "quarts" | "quart" | "qt" => return Ok("qt"),

        // Area units
        "square meters" | "square meter" | "m2" => return Ok("m2"),
        "hectares" | "hectare" | "ha" => return Ok("ha"),
        "square kilometers" | "square kilometer" | "km2" => return Ok("km2"),
        "square centimeters" | "square centimeter" | "cm2" => return Ok("cm2"),
        "square millimeters" | "square millimeter" | "mm2" => return Ok("mm2"),
        "square micrometers" | "square micrometer" | "um2" => return Ok("um2"),
        "square nanometers" | "square nanometer" | "nm2" => return Ok("nm2"),
        "square miles" | "square mile" | "mi2" => return Ok("mi2"),
        "square yards" | "square yard" | "yd2" => return Ok("yd2"),
        "square feet" | "square foot" | "ft2" => return Ok("ft2"),
        "square inches" | "square inch" | "in2" => return Ok("in2"),
        "acres" | "acre" | "ac" => return Ok("ac"),

        // Mass units
        "grams" | "gram" | "g" => return Ok("g"),
        "kilograms" | "kilogram" | "kg" => return Ok("kg"),
        "milligrams" | "milligram" | "mg" => return Ok("mg"),
        "micrograms" | "microgram" | "ug" => return Ok("ug"),
        "nanograms" | "nanogram" | "ng" => return Ok("ng"),
        "metric tons" | "metric ton" => return Ok("t metric"),
        "tons" | "ton" | "tonne" | "t" => return Ok("t"),
        "ounces" | "ounce" | "oz" => return Ok("oz"),
        "pounds" | "pound" | "lb" | "lbs" => return Ok("lb"),

        // Temperature units
        "kelvin" | "k" => return Ok("k"),
        "celsius" | "c" => return Ok("c"),
        "fahrenheit" | "f" => return Ok("f"),

        // Time units
        "seconds" | "second" | "s" => return Ok("s"),
        "milliseconds" | "millisecond" | "ms" => return Ok("ms"),
        "microseconds" | "microsecond" | "us" => return Ok("us"),
        "nanoseconds" | "nanosecond" | "ns" => return Ok("ns"),
        "hours" | "hour" | "h" => return Ok("h"),
        "minutes" | "minute" | "min" => return Ok("min"),
        "days" | "day" | "d" => return Ok("d"),
        "weeks" | "week" | "w" => return Ok("w"),
        "months" | "month" | "mo" => return Ok("mo"),
        "years" | "year" | "y" => return Ok("y"),

        // Speed units
        "meters per second" | "meter per second" | "m/s" => return Ok("m/s"),
        "kilometers per hour" | "kilometer per hour" | "km/h" => return Ok("km/h"),
        "miles per hour" | "mile per hour" | "mph" => return Ok("mph"),

        // Angle units
        "radians" | "radian" | "rad" => return Ok("rad"),
        "degrees" | "degree" | "deg" => return Ok("deg"),
        "gradians" | "gradian" | "grad" => return Ok("grad"),

        _ => return Err("unit not recognized".into()),
    }
}
