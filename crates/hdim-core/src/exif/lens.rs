#![cfg(feature = "exif")]
use super::util::{get_ascii, get_rational, get_rational_vec};
use exif::{Exif, In, Tag};

#[derive(Clone, Debug)]
pub struct LensExif {
    pub make: Option<String>,
    pub model: Option<String>,
    pub focal_length: Option<f64>,
    pub f_number_range: Option<String>,
}

pub fn get_lens_exif(exif: &Exif) -> Option<LensExif> {
    let lens_spec = get_rational_vec(exif.get_field(Tag::LensSpecification, In::PRIMARY));
    Some(LensExif {
        make: get_ascii(exif.get_field(Tag::LensMake, In::PRIMARY)),
        model: get_ascii(exif.get_field(Tag::LensModel, In::PRIMARY)),
        focal_length: get_rational(exif.get_field(Tag::FocalLength, In::PRIMARY)),
        f_number_range: lens_spec.map(|v| {
            let min = v.get(2).map(|f| format!("{:.1}", f)).unwrap_or_default();
            let max = v.get(3).map(|f| format!("{:.1}", f)).unwrap_or_default();
            format!("f/{} - f/{}", min, max)
        }),
    })
}
