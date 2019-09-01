use crate::image::Image;

pub struct State {
    pub image: Image
}

impl State {
    pub fn new() -> Self {
        State{
            image: Image::new(100,100)
        }
    }
}

