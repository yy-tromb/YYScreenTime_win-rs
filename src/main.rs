use anyhow::Result;

use iced::alignment::{Horizontal, Vertical};
use iced::font::{Family, Weight};
use iced::{event, subscription};
// use iced::multi_window;
use iced::widget::{button, column, vertical_space, Button, Column, Container, Text};
use iced::{
    executor, window, Alignment, Application, Command, Element, Event, Font, Length, Padding,
    Settings, Subscription, Theme,
};
use iced_aw::{tab_bar, tabs};

use tokio::net::windows::named_pipe;
use tokio::process;

use windows::{
    core::*,
    Win32::{
        Foundation::{self, HWND},
        System::Threading::OpenProcess,
        UI::WindowsAndMessaging::{
            EnumWindows, GetWindowThreadProcessId, MessageBoxW, MB_ICONINFORMATION, MB_OK,
            MESSAGEBOX_STYLE,
        },
    },
};
mod gui;
mod watcher;
mod windows_tools;

fn main() -> Result<()> {
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
    let mut last_value = 0;
    let mut settings = Settings::default();
    settings.antialiasing = true;
    settings.fonts = vec![
        include_bytes!("../fonts/IBMPlexSansJP-Regular.ttf")
            .as_slice()
            .into(),
        include_bytes!("../fonts/IBMPlexSansJP-Bold.ttf")
            .as_slice()
            .into(),
    ];
    settings.default_font = Font::with_name("IBM Plex Sans JP");
    settings.window = window::Settings {
        exit_on_close_request: false,
        ..window::Settings::default()
    };
    //settings.window.exit_on_close_request = false;
    gui::State::run(settings);
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
