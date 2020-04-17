use winapi::{
    um::{
        winuser::{EnumWindows, GetWindowTextW, GetWindowTextLengthW, IsWindowVisible},
        winnt::LPWSTR
    },
    shared::{minwindef::{BOOL, LPARAM}, windef::HWND, },
};

use std::error::Error;

pub struct Connection;
impl Connection {
    pub fn new() -> Result<Self, Box<dyn Error>> { Ok(Self) }
    pub fn window_titles(&self) -> Vec<String> {
        let state: Box<Vec<String>> = Box::new(Vec::new());
        let ptr = Box::into_raw(state);
        let state;
        unsafe {
            EnumWindows(Some(enumerate_windows), ptr as LPARAM);
            state = Box::from_raw(ptr);
        }
        *state
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
