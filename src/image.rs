
pub struct Image {
    width: u64,
    height: u64
}

impl Image {
    pub fn new(width: u64, height: u64) -> Self {
        Image{width,height}
    }
}
