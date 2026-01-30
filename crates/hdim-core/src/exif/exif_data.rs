use crate::exif::{CameraExif, DateTimeExif, ExposureExif, GpsExif, ImageExif, LensExif};
use exif::{Field, In, Reader, Tag, Value};
use std::io::{Read, Seek};

#[derive(Clone, Debug)]
pub struct ExifData {
    pub orientation: Option<u16>,
    pub datetime: Option<DateTimeExif>,
    pub camera: Option<CameraExif>,
    pub exposure: Option<ExposureExif>,
    pub lens: Option<LensExif>,
    pub gps: Option<GpsExif>,
    pub image: Option<ImageExif>,
}

impl ExifData {
    pub fn get_exif_data<R: Read + Seek>(reader: R) -> anyhow::Result<Self> {
        let exif = Reader::new().read_from_container(&mut std::io::BufReader::new(reader))?;

        Ok(Self {
            orientation: exif
                .get_field(Tag::Orientation, In::PRIMARY)
                .and_then(|f| f.value.get_uint(0).map(|v| v as u16)),

            datetime: Some(DateTimeExif {
                original: get_ascii(exif.get_field(Tag::DateTimeOriginal, In::PRIMARY)),
                digitized: get_ascii(exif.get_field(Tag::DateTimeDigitized, In::PRIMARY)),
                modified: get_ascii(exif.get_field(Tag::DateTime, In::PRIMARY)),
            }),

            camera: Some(CameraExif {
                make: get_ascii(exif.get_field(Tag::Make, In::PRIMARY)),
                model: get_ascii(exif.get_field(Tag::Model, In::PRIMARY)),
                software: get_ascii(exif.get_field(Tag::Software, In::PRIMARY)),
            }),

            exposure: Some(ExposureExif {
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
            }),

            lens: Some(LensExif {
                make: get_ascii(exif.get_field(Tag::LensMake, In::PRIMARY)),
                model: get_ascii(exif.get_field(Tag::LensModel, In::PRIMARY)),
                min_focal_length: None,
                max_focal_length: None,
                min_aperture: None,
                max_aperture: None,
            }),

            gps: Some(GpsExif {
                latitude: get_rational(exif.get_field(Tag::GPSLatitude, In::PRIMARY)),
                longitude: get_rational(exif.get_field(Tag::GPSLongitude, In::PRIMARY)),
                altitude: get_rational(exif.get_field(Tag::GPSAltitude, In::PRIMARY)),
                timestamp: None,
            }),

            image: None,
        })
    }
}

fn get_ascii(field: Option<&Field>) -> Option<String> {
    match &field?.value {
        Value::Ascii(v) => v.get(0).and_then(|s| String::from_utf8(s.clone()).ok()),
        _ => None,
    }
}

fn get_rational(field: Option<&Field>) -> Option<f64> {
    match &field?.value {
        Value::Rational(v) => v.get(0).map(|r| r.to_f64()),
        _ => None,
    }
}
