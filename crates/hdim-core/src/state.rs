#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tool {
    Crop,
}

#[derive(Clone, Copy, Debug)]
pub struct CropState {
    pub left: u32,
    pub right: u32,
    pub top: u32,
    pub bottom: u32,
}

impl Default for CropState {
    fn default() -> Self {
        Self {
            left: 0,
            right: 0,
            top: 0,
            bottom: 0,
        }
    }
}
