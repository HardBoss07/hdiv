use exif::{Field, Value};

pub fn get_ascii(field: Option<&Field>) -> Option<String> {
    match &field?.value {
        Value::Ascii(v) => v.get(0).and_then(|s| String::from_utf8(s.clone()).ok()),
        _ => None,
    }
}

pub fn get_rational(field: Option<&Field>) -> Option<f64> {
    match &field?.value {
        Value::Rational(v) => v.get(0).map(|r| r.to_f64()),
        _ => None,
    }
}

pub fn get_rational_vec(field: Option<&Field>) -> Option<Vec<f64>> {
    match &field?.value {
        Value::Rational(v) => Some(v.iter().map(|r| r.to_f64()).collect()),
        _ => None,
    }
}
