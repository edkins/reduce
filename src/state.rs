use crate::image::Image;
use crate::ui::UI;

pub struct State {
    pub image: Image,
    pub ui: UI
}

impl State {
    pub fn new() -> Self {
        State{
            image: Image::new(100,100),
            ui: UI::new()
        }
    }
}

