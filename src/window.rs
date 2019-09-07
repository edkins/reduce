use std::ptr::null_mut;

use winapi::shared::minwindef::DWORD;
use winapi::shared::windef::HWND;
use winapi::um::winuser::{
    CS_OWNDC,CS_HREDRAW,CS_VREDRAW,WNDCLASSW,
    CW_USEDEFAULT,
    WS_CHILD,WS_OVERLAPPEDWINDOW,WS_VISIBLE,
    CreateWindowExW,DefWindowProcW,
    RegisterClassW,
};
use winapi::um::libloaderapi::GetModuleHandleW;

use crate::win32_string;

pub fn register_class_with_menu(name: &str, menu_name: &str) {
    let name = win32_string(name);
    let menu_name = win32_string(menu_name);

    unsafe {
        let hinstance = GetModuleHandleW(null_mut());
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

        let _atom = RegisterClassW(&wnd_class);
    }
}

pub fn register_class(name: &str) {
    let name = win32_string(name);

    unsafe {
        let hinstance = GetModuleHandleW(null_mut());
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
            lpszMenuName: null_mut()
        };

        let _atom = RegisterClassW(&wnd_class);
    }
}

pub struct Window {
    hwnd: HWND
}

impl Window {
    pub fn new(class: &str, title: &str) -> Self {
        let title = win32_string(title);
        let class = win32_string(class);
        let hwnd;
        unsafe {
            hwnd = CreateWindowExW(
                0,
                class.as_ptr(),
                title.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                null_mut(),
                null_mut(),
                GetModuleHandleW(null_mut()),
                null_mut() );
        }
        Window{ hwnd }
    }
    pub fn new_child(&self, class: &str, title: &str, x: i32, y: i32, width: i32, height: i32, style: DWORD) -> Self {
        let title = win32_string(title);
        let class = win32_string(class);
        let hwnd;
        unsafe {
            hwnd = CreateWindowExW(
                0,
                class.as_ptr(),
                title.as_ptr(),
                WS_CHILD | WS_VISIBLE | style,
                x,
                y,
                width,
                height,
                self.hwnd,
                null_mut(),
                GetModuleHandleW(null_mut()),
                null_mut() );
        }
        Window{ hwnd }
    }
    pub fn get_hwnd(&self) -> HWND {
        self.hwnd
    }
}

