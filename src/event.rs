use std::mem::uninitialized;

use winapi::shared::minwindef::{UINT,WPARAM,LPARAM};
use winapi::shared::windef::{HWND,HBRUSH,RECT};
use winapi::um::winuser::{
    COLOR_WINDOW,
    MSG,
    SC_CLOSE,
    WM_COMMAND,WM_SYSCOMMAND,WM_PAINT,
    DispatchMessageW,FillRect,GetDC,GetClientRect,GetMessageW,PostQuitMessage,
    TranslateMessage
};

use crate::file_open;
use crate::state::State;
use crate::window::Window;

const FILE_OPEN:usize = 101;

unsafe fn window_proc(
    state: &mut State,
    hwnd: HWND, 
    msg: UINT, 
    wparam: WPARAM, 
    _lparam: LPARAM
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

impl Window {
    pub fn event_loop(&self, state: &mut State) {
        unsafe {
            let mut message:MSG = uninitialized();
            loop {
                if GetMessageW( &mut message as *mut MSG, self.get_hwnd(), 0, 0 ) > 0 {
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
