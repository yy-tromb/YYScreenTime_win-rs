use iced;
use windows::{
    core::*,
    Win32::{
        Foundation::{self, HWND},
        System::Threading::OpenProcess,
        UI::WindowsAndMessaging::{
            EnumWindows, MessageBoxW, MB_ICONINFORMATION, MB_OK, MESSAGEBOX_STYLE,
        },
    },
};
mod windows_tools;

fn main() -> Result<()> {
    match windows_tools::enable_visual_styles() {
        Ok(_) => (),
        Err(ref errors) => match errors {
            windows_tools::EnableVStylesErrors::Win32(win32_error) => println!("{:?}", win32_error),
            windows_tools::EnableVStylesErrors::Standard(err) => println!("{:?}", err),
        },
    };
    println!("Hello, world!");
    unsafe {
        MessageBoxW(
            HWND::default(),
            w!("Hello World!"),
            w!("YYScreenTime_win-rs"),
            MB_OK | MB_ICONINFORMATION,
        )
    };
    Ok(())
}
