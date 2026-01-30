#[derive(Clone, Debug)]
pub struct LensExif {
    pub make: Option<String>,
    pub model: Option<String>,
    pub min_focal_length: Option<f64>,
    pub max_focal_length: Option<f64>,
    pub min_aperture: Option<f64>,
    pub max_aperture: Option<f64>,
}
