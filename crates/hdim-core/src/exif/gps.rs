use super::util::{get_rational, get_rational_vec};
use exif::{Exif, In, Tag};

#[derive(Clone, Debug)]
pub struct GpsExif {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub timestamp: Option<(u8, u8, u8)>,
}

pub fn get_gps_exif(exif: &Exif) -> Option<GpsExif> {
    Some(GpsExif {
        latitude: get_rational(exif.get_field(Tag::GPSLatitude, In::PRIMARY)),
        longitude: get_rational(exif.get_field(Tag::GPSLongitude, In::PRIMARY)),
        altitude: get_rational(exif.get_field(Tag::GPSAltitude, In::PRIMARY)),
        timestamp: get_rational_vec(exif.get_field(Tag::GPSTimeStamp, In::PRIMARY)).map(|v| {
            (
                v.get(0).map_or(0, |&f| f as u8),
                v.get(1).map_or(0, |&f| f as u8),
                v.get(2).map_or(0, |&f| f as u8),
            )
        }),
    })
}
