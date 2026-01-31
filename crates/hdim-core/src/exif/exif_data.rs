#![cfg(feature = "exif")]
use super::{
    CameraExif, DateTimeExif, ExposureExif, GpsExif, ImageExif, LensExif, get_camera_exif,
    get_date_time_exif, get_exposure_exif, get_gps_exif, get_image_exif, get_lens_exif,
};
use exif::{In, Reader, Tag};
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
            datetime: get_date_time_exif(&exif),
            camera: get_camera_exif(&exif),
            exposure: get_exposure_exif(&exif),
            lens: get_lens_exif(&exif),
            gps: get_gps_exif(&exif),
            image: get_image_exif(&exif),
        })
    }
}
