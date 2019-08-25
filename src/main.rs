use std::iter::once;
use std::mem::uninitialized;
use std::ptr::null_mut;

use winapi::shared::minwindef::{ATOM,HMODULE,UINT,WPARAM,LPARAM,LRESULT};
use winapi::shared::windef::{HWND};
use winapi::um::winuser::{
    CS_OWNDC,CS_HREDRAW,CS_VREDRAW,WNDCLASSW,
    CW_USEDEFAULT,
    MSG,
    WM_DESTROY,
    WS_OVERLAPPEDWINDOW,WS_VISIBLE,
    CreateWindowExW,DefWindowProcW,DispatchMessageW,GetMessageW,PostQuitMessage,RegisterClassW,TranslateMessage};
use winapi::um::libloaderapi::GetModuleHandleW;

fn win32_string(value : &str) -> Vec<u16> {
    value.chars().map(|c|c as u16).chain( once( 0 ) ).collect()
}

struct WndClass {
    hinstance: HMODULE,
    _atom: ATOM,
    name: Vec<u16>
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
            0
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam)
    }
}

impl WndClass {
    fn new(name: &str) -> Self {
        let name = win32_string(name);

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
                lpszMenuName: null_mut(),
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
    let class = WndClass::new("reduce");
    let window = Window::new(&class, "Reduce Images");
    window.event_loop();
}
