use std::char::{decode_utf16, REPLACEMENT_CHARACTER};
use std::fs;
use std::path::Path;
use std::slice;
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
struct My_WIN32_ERROR(WIN32_ERROR);
impl std::fmt::Display for My_WIN32_ERROR {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Win32 Error: {:?}", &self.0)
    }
}
impl std::error::Error for My_WIN32_ERROR {}
enum StrAndWin32Error {
    ReasonString(String),
    Win32(WIN32_ERROR),
}

/*  This function is referred to
[Native Windows GUI](https://github.com/gabdube/native-windows-gui/tree/master?tab=readme-ov-file) of
native_windows_gui::enable_visual_styles*/
pub fn enable_visual_styles() -> std::result::Result<(), My_WIN32_ERROR> {
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
        return Err(My_WIN32_ERROR(unsafe { GetLastError() }));
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
    let get_tmp_file_result =
        unsafe { GetTempFileNameW(manifest_dir_raw, prefix, 0, &mut tmp_path) };
    if get_tmp_file_result == 0 {
        return Err(My_WIN32_ERROR(unsafe { GetLastError() }));
    }
    let manifest_path_utf16 = PCWSTR::from_raw(&tmp_path as *const u16);
    let manifest_path = decode_utf16_with_capacity(&tmp_path, MAX_PATH_USIZE);
    if let Err(err) = fs::write(Path::new(manifest_path), MANIFEST_CONTENT) {
        dbg!(err);
    };
    println!("{}", manifest_path);
    let mut activation_cookie: usize = 0;
    let mut act_ctx = ACTCTXW {
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
        let handle = match CreateActCtxW(&mut act_ctx) {
            Ok(handle) => handle,
            Err(err) => {
                dbg!(unsafe { GetLastError() });
                return Err(My_WIN32_ERROR(unsafe { GetLastError() }));
            }
        };
        if let Err(err) = ActivateActCtx(handle, &mut activation_cookie) {
            return Err(My_WIN32_ERROR(unsafe { GetLastError() }));
        };
    }
    unsafe {
        MessageBoxW(
            HWND::default(),
            w!("Hello World!"),
            w!("YYScreenTime_win-rs"),
            MB_OK | MB_ICONINFORMATION,
        )
    };
    fs::remove_file(&manifest_path);
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

#[test]
fn call_enable_visual_styles() {
    enable_visual_styles();
    unsafe {
        MessageBoxW(
            HWND::default(),
            w!("Hello World!"),
            w!("YYScreenTime_win-rs"),
            MB_OK | MB_ICONINFORMATION,
        )
    };
}
