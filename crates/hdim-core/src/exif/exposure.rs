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
