mod error;
mod event;
mod image;
mod open;
mod state;
mod ui;
mod window;

use std::iter::once;
use std::ptr::null_mut;

use winapi::shared::minwindef::FALSE;
use winapi::shared::windef::{HWND};
use winapi::um::winuser::{InvalidateRect};

use crate::event::event_loop;
use crate::image::Image;
use crate::open::show_file_open_dialog;
use crate::state::State;
use crate::ui::UI;

pub fn win32_string(value : &str) -> Vec<u16> {
    value.chars().map(|c|c as u16).chain( once( 0 ) ).collect()
}

pub fn from_win32_string(value: &[u16]) -> Option<String> {
    let mut result = String::new();
    for ch in value {
        if *ch == 0 {
            return Some(result);
        }
        result.push(std::char::from_u32(*ch as u32)?);
    }
    Some(result)
}

fn file_open(state: &mut State, hwnd: HWND) {
    let filename = show_file_open_dialog(hwnd);
    if filename.is_some() {
        let image_res = Image::load(&filename.unwrap());
        if image_res.is_ok() {
            state.image = image_res.unwrap();
            unsafe {
                InvalidateRect(hwnd, null_mut(), FALSE);
            }
        } else {
            println!("{:?}", image_res.unwrap_err());
        }
    }
}

fn main() {
    UI::register_classes();
    let mut state = State::new();
    event_loop(&mut state);
}
