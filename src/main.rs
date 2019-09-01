mod error;
mod image;
mod open;
mod state;

use std::iter::once;
use std::mem::uninitialized;
use std::ptr::null_mut;

use winapi::shared::minwindef::{ATOM,HMODULE,UINT,WPARAM,LPARAM,TRUE};
use winapi::shared::windef::{HWND,HBRUSH,RECT};
use winapi::um::winuser::{
    COLOR_WINDOW,
    CS_OWNDC,CS_HREDRAW,CS_VREDRAW,WNDCLASSW,
    CW_USEDEFAULT,
    MSG,
    SC_CLOSE,
    WM_COMMAND,WM_SYSCOMMAND,WM_PAINT,
    WS_OVERLAPPEDWINDOW,WS_VISIBLE,
    CreateWindowExW,DefWindowProcW,DispatchMessageW,FillRect,
    GetDC,GetMessageW,GetClientRect,InvalidateRect,
    PostQuitMessage,RegisterClassW,TranslateMessage,
};
use winapi::um::libloaderapi::GetModuleHandleW;

use crate::image::Image;
use crate::open::show_file_open_dialog;
use crate::state::State;

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

struct WndClass {
    hinstance: HMODULE,
    _atom: ATOM,
    name: Vec<u16>
}

const FILE_OPEN:usize = 101;

fn file_open(state: &mut State, hwnd: HWND) {
    let filename = show_file_open_dialog(hwnd);
    if filename.is_some() {
        let image_res = Image::load(&filename.unwrap());
        if image_res.is_ok() {
            state.image = image_res.unwrap();
            unsafe {
                InvalidateRect(hwnd, null_mut(), TRUE);
            }
        } else {
            println!("{:?}", image_res.unwrap_err());
        }
    }
}

unsafe fn window_proc(
    state: &mut State,
    hwnd: HWND, 
    msg: UINT, 
    wparam: WPARAM, 
    lparam: LPARAM
)
{
    match msg {
        WM_COMMAND => {
            match wparam & 0xffff {
                FILE_OPEN => file_open(state, hwnd),
                _ => println!("WM_COMMAND. wparam low word = {}", wparam & 0xffff)
            }
        }
        WM_SYSCOMMAND => {
            match wparam & 0xffff {
                SC_CLOSE => PostQuitMessage(0),
                _ => {}
            }
        }
        WM_PAINT => {
            let dc = GetDC(hwnd);
            let mut rect = RECT{left:0, top:0, right: 0, bottom: 0};
            GetClientRect(hwnd, &mut rect);
            FillRect(dc, &rect, (COLOR_WINDOW+1) as HBRUSH);
            state.image.paint_to_dc(dc);
        }
        _ => {}
    }
}

impl WndClass {
    fn new(name: &str, menu_name: &str) -> Self {
        let name = win32_string(name);
        let menu_name = win32_string(menu_name);

        let atom;
        let hinstance;

        unsafe {
            hinstance = GetModuleHandleW(null_mut());
            let wnd_class = WNDCLASSW {
                style : CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc : Some(DefWindowProcW),
                hInstance : hinstance,
                lpszClassName : name.as_ptr(),
                cbClsExtra : 0,
                cbWndExtra : 0,
                hIcon: null_mut(),
                hCursor: null_mut(),
                hbrBackground: null_mut(),
                lpszMenuName: menu_name.as_ptr()
            };

            atom = RegisterClassW(&wnd_class);
        }

        WndClass{ _atom: atom, name: name, hinstance: hinstance }
    }
}

struct Window {
    hwnd: HWND
}

impl Window {
    fn new(class: &WndClass, title: &str) -> Self {
        let title = win32_string(title);
        let hwnd;
        unsafe {
            hwnd = CreateWindowExW(
                0,
                class.name.as_ptr(),
                title.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                null_mut(),
                null_mut(),
                class.hinstance,
                null_mut() );
        }
        Window{ hwnd: hwnd }
    }

    fn event_loop(&self, state: &mut State) {
        unsafe {
            let mut message:MSG = uninitialized();
            loop {
                if GetMessageW( &mut message as *mut MSG, self.hwnd, 0, 0 ) > 0 {
                    TranslateMessage( &message as *const MSG );
                    window_proc( state, message.hwnd, message.message, message.wParam, message.lParam );
                    DispatchMessageW( &message as *const MSG );
                } else {
                    break;
                }
            }
        }
    }
}

fn main() {
    let class = WndClass::new("reduce","menubar");
    let window = Window::new(&class, "Reduce Images");
    let mut state = State::new();
    window.event_loop(&mut state);
}
