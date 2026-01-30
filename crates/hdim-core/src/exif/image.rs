use super::util::get_rational;
use exif::{Exif, In, Tag};

#[derive(Clone, Debug)]
pub struct ImageExif {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub resolution_unit: Option<u16>,
    pub x_resolution: Option<f64>,
    pub y_resolution: Option<f64>,
}

pub fn get_image_exif(exif: &Exif) -> Option<ImageExif> {
    Some(ImageExif {
        width: exif
            .get_field(Tag::ImageWidth, In::PRIMARY)
            .and_then(|f| f.value.get_uint(0)),
        height: exif
            .get_field(Tag::ImageLength, In::PRIMARY)
            .and_then(|f| f.value.get_uint(0)),
        resolution_unit: exif
            .get_field(Tag::ResolutionUnit, In::PRIMARY)
            .and_then(|f| f.value.get_uint(0).map(|v| v as u16)),
        x_resolution: get_rational(exif.get_field(Tag::XResolution, In::PRIMARY)),
        y_resolution: get_rational(exif.get_field(Tag::YResolution, In::PRIMARY)),
    })
}
