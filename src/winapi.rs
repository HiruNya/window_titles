use winapi::{
    um::{
        winuser::{EnumWindows, GetWindowTextW, GetWindowTextLengthW, IsWindowVisible},
        winnt::LPWSTR
    },
    shared::{minwindef::{BOOL, LPARAM}, windef::HWND},
};

use crate::{ConnectionTrait, Result};

pub struct Connection;
impl ConnectionTrait for Connection {
    fn new() -> Result<Self> { Ok(Self) }
    fn window_titles(&self) -> Result<Vec<String>> {
        let state: Box<Vec<String>> = Box::new(Vec::new());
        let ptr = Box::into_raw(state);
        let state;
        unsafe {
            EnumWindows(Some(enumerate_windows), ptr as LPARAM);
            state = Box::from_raw(ptr);
        }
        Ok(*state)
    }
}

unsafe extern "system" fn enumerate_windows(window: HWND, state: LPARAM) -> BOOL {
    if IsWindowVisible(window) == 0 { return true.into() }
    let state = state as *mut Vec<String>;
    let length = GetWindowTextLengthW(window);
    let mut title: Vec<u16> = vec![0; length as usize];
    if GetWindowTextW(window, title.as_mut_ptr() as LPWSTR, length+1) != 0 {
        if let Ok(title) = String::from_utf16(title[0..(length as usize)].as_ref()) {
            (*state).push(title);
        }
    }
    true.into()
}
