use tokio::net::windows::named_pipe;
use tokio::process;

use windows::{
    Win32::{
        Foundation::{self, HWND},
        System::Threading::OpenProcess,
        UI::WindowsAndMessaging::{
            EnumWindows, GetWindowThreadProcessId, MB_ICONINFORMATION, MB_OK, MESSAGEBOX_STYLE,
            MessageBoxW,
        },
    },
    core::*,
};
mod gui;
mod watcher;
mod windows_tools;

fn main() -> anyhow::Result<()> {
    windows_tools::enable_visual_styles()?;
    println!("Hello, World!");
    unsafe {
        MessageBoxW(
            HWND::default(),
            w!("Hello World!"),
            w!("YYScreenTime_win-rs"),
            MB_OK | MB_ICONINFORMATION,
        )
    };
    gui::window_open()?;
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
