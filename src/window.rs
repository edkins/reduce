use std::ptr::null_mut;

use winapi::shared::minwindef::{ATOM,HMODULE};
use winapi::shared::windef::{HWND};
use winapi::um::winuser::{
    CS_OWNDC,CS_HREDRAW,CS_VREDRAW,WNDCLASSW,
    CW_USEDEFAULT,
    WS_OVERLAPPEDWINDOW,WS_VISIBLE,
    CreateWindowExW,DefWindowProcW,
    RegisterClassW,
};
use winapi::um::libloaderapi::GetModuleHandleW;

use crate::win32_string;

pub struct WndClass {
    hinstance: HMODULE,
    _atom: ATOM,
    name: Vec<u16>
}

impl WndClass {
    pub fn new(name: &str, menu_name: &str) -> Self {
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

pub struct Window {
    hwnd: HWND
}

impl Window {
    pub fn new(class: &WndClass, title: &str) -> Self {
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
    pub fn get_hwnd(&self) -> HWND {
        self.hwnd
    }
}

