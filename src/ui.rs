use std::ptr::null_mut;

use winapi::um::winuser::{WS_HSCROLL,WS_VSCROLL,SWP_NOZORDER,
    SetWindowPos};

use crate::window::{Window,register_class,register_class_with_menu};

pub struct UI {
    pub main: Window,
    pub image: Window
}

impl UI {
    pub fn register_classes() {
        register_class_with_menu("main","menubar");
        register_class("panel");
    }
    pub fn new() -> Self {
        let main = Window::new("main","Reduce Images");
        let image = main.new_child("panel","",0,0,500,500,WS_HSCROLL|WS_VSCROLL);
        UI { main, image }
    }
    pub fn resize_inner(&mut self, width: i32, height: i32) {
        unsafe {
            SetWindowPos(
                self.image.get_hwnd(),
                null_mut(),
                0,
                0,
                width/2,
                height,
                SWP_NOZORDER);
        }
    }
}
