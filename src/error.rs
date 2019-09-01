#[derive(Debug)]
pub enum Er {
    UnknownImageFormat(String),
    UnknownColourFormat(image::ColorType),
    IO(std::io::Error),
    ImageDecoding(image::ImageError)
}

impl From<std::io::Error> for Er {
    fn from(e: std::io::Error) -> Self {
        Er::IO(e)
    }
}

impl From<image::ImageError> for Er {
    fn from(e: image::ImageError) -> Self {
        Er::ImageDecoding(e)
    }
}
