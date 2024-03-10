use windows::{
    core::*,
    Win32::{
        Foundation::{self, HWND},
        System::Threading::OpenProcess,
        UI::WindowsAndMessaging::{EnumWindows, MessageBoxW, MB_OK, MESSAGEBOX_STYLE},
    },
};
mod windows_tools;

fn main() -> Result<()> {
    println!("Hello, world!");
    unsafe {
        MessageBoxW(
            HWND::default(),
            w!("Hello World!"),
            w!("YYScreenTime_win-rs"),
            MB_OK,
        )
    };
    Ok(())
}
