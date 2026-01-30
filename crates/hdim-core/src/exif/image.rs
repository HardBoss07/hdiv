#[derive(Clone, Debug)]
pub struct ImageExif {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub color_space: Option<u16>,
}
