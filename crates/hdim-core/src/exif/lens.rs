use super::util::{get_ascii, get_rational_vec};
use exif::{Exif, In, Tag};

#[derive(Clone, Debug)]
pub struct LensExif {
    pub make: Option<String>,
    pub model: Option<String>,
    pub min_focal_length: Option<f64>,
    pub max_focal_length: Option<f64>,
    pub min_aperture: Option<f64>,
    pub max_aperture: Option<f64>,
}

pub fn get_lens_exif(exif: &Exif) -> Option<LensExif> {
    let lens_spec = exif.get_field(Tag::LensSpecification, In::PRIMARY);
    Some(LensExif {
        make: get_ascii(exif.get_field(Tag::LensMake, In::PRIMARY)),
        model: get_ascii(exif.get_field(Tag::LensModel, In::PRIMARY)),
        min_focal_length: get_rational_vec(lens_spec).and_then(|v| v.get(0).copied()),
        max_focal_length: get_rational_vec(lens_spec).and_then(|v| v.get(1).copied()),
        min_aperture: get_rational_vec(lens_spec).and_then(|v| v.get(2).copied()),
        max_aperture: get_rational_vec(lens_spec).and_then(|v| v.get(3).copied()),
    })
}
