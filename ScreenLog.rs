// Screen Logger in Rust
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::time::Duration;
use winapi::{
    shared::{
        minwindef::{BOOL, DWORD, HBITMAP, HDC, LPARAM},
        ntdef::LPCWSTR,
        windef::{HDC__, HWND, RECT},
    },
    um::{
        wingdi::{
            BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject,
            GetDeviceCaps, GetObjectW, SelectObject, BITMAP, COLORREF, SRCCOPY,
        },
        winuser::GetDesktopWindow,
    },
};

const LOG_FILE_NAME: &str = "screenlog.txt";

fn log_screen() {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE_NAME)
        .unwrap();
    let hwnd = unsafe { GetDesktopWindow() };
    let mut rect: RECT = Default::default();
    let hdc_screen = unsafe { GetDC(hwnd) };
    let hdc = unsafe { CreateCompatibleDC(hdc_screen) };
    let width = unsafe { GetDeviceCaps(hdc_screen, 8) };
let height = unsafe { GetDeviceCaps(hdc_screen, 10) };
let hbitmap = unsafe { SelectObject(hdc, CreateCompatibleBitmap(hdc_screen, width, height)) };
let mut bitmap: BITMAP = Default::default();
let _ = unsafe { GetObjectW(hbitmap as *mut _, std::mem::size_of::<BITMAP>(), &mut bitmap) };
let mut writer = BufWriter::new(file);
loop {
    std::thread::sleep(Duration::from_secs(10));
    unsafe { BitBlt(hdc, 0, 0, width, height, hdc_screen, 0, 0, SRCCOPY) };
    let mut written: DWORD = 0;
    let mut data: Vec<u8> = vec![0; (bitmap.bmWidth * bitmap.bmHeight * 4) as usize];
    let lp_bits: *mut u8 = data.as_mut_ptr();
    let bmi = std::mem::zeroed::<winapi::um::wingdi::BITMAPINFO>();
    let bmi_header = bmi.bmiHeader;
    bmi_header.biSize = std::mem::size_of::<winapi::um::wingdi::BITMAPINFOHEADER>() as u32;
    bmi_header.biWidth = bitmap.bmWidth;
    bmi_header.biHeight = bitmap.bmHeight;
    bmi_header.biPlanes = 1;
    bmi_header.biBitCount = 32;
    bmi_header.biCompression = winapi::um::wingdi::BI_RGB;
    let result = unsafe {
        GetDIBits(
            hdc_screen,
            hbitmap as winapi::um::winnt::HANDLE,
            0,
            bitmap.bmHeight as u32,
            lp_bits as *mut _,
            &bmi,
            winapi::um::winnt::DIB_RGB_COLORS,
        )
    };
    if result == 0 {
        break;
    }
    let color_bytes = 4;
    for row in data.chunks((bitmap.bmWidth * color_bytes) as usize) {
        writer.write_all(row).unwrap();
    }
    writer.flush().unwrap();
}
unsafe {
    DeleteObject(hbitmap as winapi::shared::windef::HGDIOBJ);
    DeleteDC(hdc);
    DeleteDC(hdc_screen);
}
}
fn main() {
log_screen();
}

