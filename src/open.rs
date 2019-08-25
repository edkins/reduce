use std::mem::size_of;
use std::ptr::null_mut;

use winapi::shared::windef::{HWND};
use winapi::um::commdlg::{OPENFILENAMEW,OFN_EXPLORER,OFN_FILEMUSTEXIST,OFN_HIDEREADONLY,OFN_PATHMUSTEXIST,GetOpenFileNameW};
use winapi::um::libloaderapi::GetModuleHandleW;

use crate::from_win32_string;

fn to_win32_filter(items: &[(&str,&str)]) -> Vec<u16> {
    if items.len() == 0 {
        panic!("to_win32_filter: needs at least one item");
    }
    let mut result = vec![];
    for (label,pattern) in items {
        for ch in label.chars() {
            result.push(ch as u16)
        }
        result.push(0);
        for ch in pattern.chars() {
            result.push(ch as u16)
        }
        result.push(0);
    }
    result.push(0);
    result
}

pub fn show_file_open_dialog(owner: HWND) -> Option<String> {
    unsafe {
        let filter = to_win32_filter(&[("Images","*.JPG;*.JPEG;*.GIF;*.PNG")]);
        let mut buffer = vec![0u16; 1024];
        let mut openfilename = OPENFILENAMEW {
            lStructSize: size_of::<OPENFILENAMEW>() as u32,
            hwndOwner: owner,
            hInstance: GetModuleHandleW(null_mut()),
            lpstrFilter: filter.as_ptr(),
            lpstrCustomFilter: null_mut(),
            nMaxCustFilter: 0,
            nFilterIndex: 1,
            lpstrFile: buffer.as_mut_ptr(),
            nMaxFile: buffer.len() as u32,
            lpstrFileTitle: null_mut(),
            nMaxFileTitle: 0,
            lpstrInitialDir: null_mut(),
            lpstrTitle: null_mut(),
            Flags: OFN_EXPLORER | OFN_FILEMUSTEXIST | OFN_HIDEREADONLY | OFN_PATHMUSTEXIST,
            nFileOffset: 0,
            nFileExtension: 0,
            lpstrDefExt: null_mut(),
            lCustData: 0,
            lpfnHook: None,
            lpTemplateName: null_mut(),
            pvReserved: null_mut(),
            dwReserved: 0,
            FlagsEx: 0
        };
        let result = GetOpenFileNameW(&mut openfilename);
        if result == 0 {
            return None;
        } else {
            return from_win32_string(&buffer)
        }
    }
}
