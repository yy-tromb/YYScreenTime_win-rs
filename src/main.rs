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
    /*match windows_tools::enable_visual_styles() {
        Ok(t) => t,
        Err(_) => ,
    };*/
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
