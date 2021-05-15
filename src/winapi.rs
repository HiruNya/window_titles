use winapi::{
    shared::{minwindef::{BOOL, DWORD, HMODULE, LPARAM, LPDWORD}, windef::HWND, },
    um::{
        handleapi::CloseHandle,
        processthreadsapi::OpenProcess,
        psapi::{EnumProcesses, GetModuleBaseNameW},
        winuser::{EnumWindows, GetWindowTextW, GetWindowTextLengthW, GetWindowThreadProcessId, IsWindowVisible},
        winnt::{LPWSTR, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ},
    },
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
            EnumWindows(Some(enumerate_window_titles), ptr as LPARAM);
            state = Box::from_raw(ptr);
        }
        Ok(*state)
    }
}
impl Connection {

    pub fn windows(&self) -> Box<Vec<Window>> {
        let state: Box<Vec<Window>> = Box::new(Vec::new());
        let ptr = Box::into_raw(state);
        let state;
        unsafe {
            EnumWindows(Some(enumerate_windows), ptr as LPARAM);
            state = Box::from_raw(ptr);
        }
        state
    }

    pub fn processes(&self) -> Vec<Process> {
        let mut processes: Box<[DWORD; 1024]> = Box::new([0 as DWORD; 1024]);
        let mut byte_num: u32 = 0;
        unsafe {
            EnumProcesses(processes.as_mut_ptr(), 1024, &mut byte_num as LPDWORD);
        }
        let process_num = byte_num / 4; // DWORD is 32 bits i.e. 4 bytes
        processes.iter().take(process_num as usize).map(|pid| Process(*pid)).collect()
    }
}

pub struct Window(pub HWND);
impl Window {
    pub fn name(&self) -> Result<String> {
        unsafe {
            let length = GetWindowTextLengthW(self.0);
            let mut title: Vec<u16> = vec![0; length as usize];
            if GetWindowTextW(self.0, title.as_mut_ptr() as LPWSTR, length+1) != 0 {
                return Ok(String::from_utf16(title[0..(length as usize)].as_ref())?);
            }
        }
        Ok(String::new())
    }

    pub fn process(&self) -> Process {
        let mut pid = 0;
        unsafe {
            let _thread = GetWindowThreadProcessId(self.0, &mut pid as LPDWORD);
        }
        Process(pid)
    }
}

pub struct Process(pub u32);
impl Process {
    pub fn name(&self) -> String {
        unsafe {
            let handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false as BOOL, self.0);
            let mut buffer = Box::new([0 as u16; 512]);
            GetModuleBaseNameW(handle, 0 as HMODULE, buffer.as_mut_ptr() as LPWSTR, 512 as DWORD);
            CloseHandle(handle);
            let mut name = String::from_utf16(buffer.as_ref()).expect("Whoops");
            let actual_size = name.find("\0").unwrap_or(512);
            name.truncate(actual_size);
            name
        }
    }
}

unsafe extern "system" fn enumerate_windows(window: HWND, state: LPARAM) -> BOOL {
    if IsWindowVisible(window) == 0 { return true.into() }
    let state = state as *mut Vec<Window>;
    if let Some(state) = state.as_mut() {
        state.push(Window(window));
    }
    true.into()
}

unsafe extern "system" fn enumerate_window_titles(window: HWND, state: LPARAM) -> BOOL {
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
