#[derive(Clone, Debug)]
pub struct GpsExif {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
    pub timestamp: Option<(u8, u8, u8)>,
}
