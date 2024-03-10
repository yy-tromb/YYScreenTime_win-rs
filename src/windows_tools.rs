use windows::{
    core::*,
    Win32::{
        Foundation::{self, HWND},
        System::Threading::OpenProcess,
        UI::WindowsAndMessaging::{EnumWindows, MessageBoxW, MB_OK, MESSAGEBOX_STYLE},
    },
};

pub fn enable_visual_styles() {}
