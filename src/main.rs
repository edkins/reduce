mod open;

use std::iter::once;
use std::mem::uninitialized;
use std::ptr::null_mut;

use winapi::shared::minwindef::{ATOM,HMODULE,UINT,WPARAM,LPARAM,LRESULT};
use winapi::shared::windef::{HWND};
use winapi::um::winuser::{
    CS_OWNDC,CS_HREDRAW,CS_VREDRAW,WNDCLASSW,
    CW_USEDEFAULT,
    MSG,
    WM_COMMAND,WM_DESTROY,
    WS_OVERLAPPEDWINDOW,WS_VISIBLE,
    CreateWindowExW,DefWindowProcW,DispatchMessageW,GetMessageW,PostQuitMessage,RegisterClassW,TranslateMessage};
use winapi::um::libloaderapi::GetModuleHandleW;

use crate::open::show_file_open_dialog;

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

fn file_open(hwnd: HWND) {
    let filename = show_file_open_dialog(hwnd);
    println!("Filename: {:?}", filename);
}

unsafe extern "system" fn window_proc(
    hwnd: HWND, 
    msg: UINT, 
    wparam: WPARAM, 
    lparam: LPARAM
) -> LRESULT
{
    match msg {
        WM_DESTROY => {
            PostQuitMessage(0);
        }
        WM_COMMAND => {
            match wparam & 0xffff {
                FILE_OPEN => file_open(hwnd),
                _ => println!("WM_COMMAND. wparam low word = {}", wparam & 0xffff)
            }
        }
        _ => {}
    }
    DefWindowProcW(hwnd, msg, wparam, lparam)
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
                lpfnWndProc : Some(window_proc),
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

    fn event_loop(&self) {
        unsafe {
            let mut message:MSG = uninitialized();
            loop {
                if GetMessageW( &mut message as *mut MSG, self.hwnd, 0, 0 ) > 0 {
                    TranslateMessage( &message as *const MSG );
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
    window.event_loop();
}
