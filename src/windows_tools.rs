use anyhow::{anyhow, Result};
use iced::futures::future::ok;
use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::fs;
use std::path::Path;
use std::ptr::write;
use windows::Win32::Foundation::HMODULE;
use windows::Win32::UI::WindowsAndMessaging::MB_ICONINFORMATION;
use windows::{
    core::*,
    Win32::{
        Foundation::{self, GetLastError, HWND, MAX_PATH, WIN32_ERROR},
        Storage::FileSystem::{GetTempFileNameW, GetTempPath2W},
        System::{
            ApplicationInstallationAndServicing::{ActivateActCtx, CreateActCtxW, ACTCTXW},
            Threading::OpenProcess,
        },
        UI::WindowsAndMessaging::{EnumWindows, MessageBoxW, MB_OK, MESSAGEBOX_STYLE},
    },
};

#[derive(Debug)]
pub enum EnableVStylesErrors {
    Win32(WIN32_ERROR),
    Standard(std::io::Error),
}

impl std::fmt::Display for EnableVStylesErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EnableVStylesErrors::Win32(win32_error) => write!(f, "Win32 Error: {:?}", win32_error),
            EnableVStylesErrors::Standard(err) => write!(f, "{:?}", err),
        }
    }
}
impl std::error::Error for EnableVStylesErrors {}

#[derive(Debug)]
struct MyWin32Err(u32);
impl MyWin32Err {
    fn new(win32_err: WIN32_ERROR) -> Self {
        Self(win32_err.0)
    }
}
impl std::fmt::Display for MyWin32Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Win32 Error: {:?}", &self.0)
    }
}
impl std::error::Error for MyWin32Err {}

#[inline]
fn win32_to_anyhow(win32_error: WIN32_ERROR) -> anyhow::Error {
    anyhow!("Win32 Error: {}", win32_error.0)
}

/*
This function is referred to
[Native Windows GUI](https://github.com/gabdube/native-windows-gui) of
native_windows_gui::enable_visual_styles
(in native_windows_gui/src/win32/mod.rs)
Thanks to [gabdube](https://github.com/gabdube/) and conntoributers.
*/
pub fn enable_visual_styles() -> Result<()> {
    const MAX_PATH_USIZE: usize = MAX_PATH as usize;
    const MANIFEST_CONTENT: &str = r#"
<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
    <description>YYScreenTime_win-rs comctl32 manifest</description>
    <dependency>
        <dependentAssembly>
            <assemblyIdentity type="win32" name="Microsoft.Windows.Common-Controls" version="6.0.0.0" processorArchitecture="*" publicKeyToken="6595b64144ccf1df" language="*" /> 
        </dependentAssembly>
    </dependency>
</assembly>
"#;
    const ACTCTX_FLAG_SET_PROCESS_DEFAULT: u32 = 0x010;
    let mut tmp_dir = [0u16; MAX_PATH_USIZE + 1];
    if unsafe { GetTempPath2W(Some(&mut tmp_dir)) } == 0 {
        return Err(win32_to_anyhow(unsafe { GetLastError() }));
    }
    /*if tmp_dir.len() > MAX_PATH_USIZE - 14 {
        println!(
            "{},{}",
            decode_utf16_with_capacity(&tmp_dir, MAX_PATH_USIZE),
            tmp_dir.len()
        );
        return Err(StrAndWin32Error::ReasonString(
            "Tmporary directory path is too long. ".into(),
        ));
    }*/
    let mut tmp_path = [0u16; MAX_PATH_USIZE];
    let prefix = w!("nwg");
    let manifest_dir_raw = PCWSTR::from_raw(&tmp_dir as *const u16);
    if unsafe { GetTempFileNameW(manifest_dir_raw, prefix, 0, &mut tmp_path) } == 0 {
        return Err(win32_to_anyhow(unsafe { GetLastError() }));
    };
    let manifest_path_utf16 = PCWSTR::from_raw(&tmp_path as *const u16);
    let manifest_path_len = tmp_path
        .iter()
        .position(|&i| i == 0)
        .unwrap_or(tmp_path.len());
    let manifest_path =
        decode_utf16_with_capacity(&tmp_path[0..manifest_path_len], manifest_path_len);
    fs::write(Path::new(&manifest_path), MANIFEST_CONTENT)?;
    let mut activation_cookie: usize = 0;
    let act_ctx = ACTCTXW {
        cbSize: std::mem::size_of::<ACTCTXW>() as u32,
        dwFlags: ACTCTX_FLAG_SET_PROCESS_DEFAULT,
        lpSource: manifest_path_utf16,
        wProcessorArchitecture: 0,
        wLangId: 0,
        lpAssemblyDirectory: w!(""),
        lpResourceName: w!(""),
        lpApplicationName: w!(""),
        hModule: HMODULE::default(),
    };
    unsafe {
        let handle = match CreateActCtxW(&act_ctx) {
            Ok(handle) => handle,
            Err(_err) => {
                if GetLastError() == WIN32_ERROR(0) {
                    fs::remove_file(&manifest_path)?;
                    return Ok(());
                } else {
                    return Err(win32_to_anyhow(GetLastError()));
                };
            }
        };
        if let Err(_err) = ActivateActCtx(handle, &mut activation_cookie) {
            return Err(win32_to_anyhow(GetLastError()));
        };
    };
    fs::remove_file(&manifest_path)?;
    Ok(())
}

fn decode_utf16_with_capacity(source: &[u16], capacity: usize) -> String {
    let decoded = decode_utf16(source.iter().cloned());
    let mut buf = String::with_capacity(capacity);
    for r in decoded {
        buf.push(r.unwrap_or(REPLACEMENT_CHARACTER));
    }
    buf
}
