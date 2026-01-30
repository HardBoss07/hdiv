use super::util::get_rational;
use exif::{Exif, In, Tag};

#[derive(Clone, Debug)]
pub struct ExposureExif {
    pub exposure_time: Option<f64>,
    pub f_number: Option<f64>,
    pub iso: Option<u32>,
    pub exposure_bias: Option<f64>,
    pub metering_mode: Option<u16>,
    pub flash: Option<u16>,
    pub focal_length: Option<f64>,
    pub white_balance: Option<u16>,
}

pub fn get_exposure_exif(exif: &Exif) -> Option<ExposureExif> {
    Some(ExposureExif {
        exposure_time: get_rational(exif.get_field(Tag::ExposureTime, In::PRIMARY)),
        f_number: get_rational(exif.get_field(Tag::FNumber, In::PRIMARY)),
        iso: exif
            .get_field(Tag::ISOSpeed, In::PRIMARY)
            .and_then(|f| f.value.get_uint(0).map(|v| v as u32)),
        exposure_bias: get_rational(exif.get_field(Tag::ExposureBiasValue, In::PRIMARY)),
        metering_mode: exif
            .get_field(Tag::MeteringMode, In::PRIMARY)
            .and_then(|f| f.value.get_uint(0).map(|v| v as u16)),
        flash: exif
            .get_field(Tag::Flash, In::PRIMARY)
            .and_then(|f| f.value.get_uint(0).map(|v| v as u16)),
        focal_length: get_rational(exif.get_field(Tag::FocalLength, In::PRIMARY)),
        white_balance: exif
            .get_field(Tag::WhiteBalance, In::PRIMARY)
            .and_then(|f| f.value.get_uint(0).map(|v| v as u16)),
    })
}
