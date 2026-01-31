#![cfg(feature = "exif")]
use super::util::get_ascii;
use exif::{Exif, In, Tag};

#[derive(Clone, Debug)]
pub struct DateTimeExif {
    pub original: Option<String>,
    pub digitized: Option<String>,
    pub modified: Option<String>,
}

pub fn get_date_time_exif(exif: &Exif) -> Option<DateTimeExif> {
    Some(DateTimeExif {
        original: get_ascii(exif.get_field(Tag::DateTimeOriginal, In::PRIMARY)),
        digitized: get_ascii(exif.get_field(Tag::DateTimeDigitized, In::PRIMARY)),
        modified: get_ascii(exif.get_field(Tag::DateTime, In::PRIMARY)),
    })
}
