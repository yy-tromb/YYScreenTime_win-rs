use iced::alignment::{Horizontal, Vertical};
use iced::font::{Family, Weight};
use iced::widget::{button, column, vertical_space, Button, Column, Container, Text};
use iced::{
    executor, window, Alignment, Application, Command, Element, Event, Font, Length, Padding,
    Settings, Subscription, Theme,
};

use anyhow::Result;
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
use iced::event;

// アプリケーションの状態を定義する構造体
struct State {
    // 数値を保持するフィールド
    value: i32,
    // ボタンの状態を保持するフィールド
    increment_button: button::State,
}

// メッセージを定義する列挙体
#[derive(Debug, Clone)]
enum Message {
    IncrementPressed,
    WindowClosed,
    WindowEventOccured(Event),
}

impl Application for State {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            State {
                value: 0,
                increment_button: button::State::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Counter")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
                Command::none()
            }
            Message::WindowClosed => {
                // ウィンドウが閉じられたときの処理はここでは行わない
                Command::none()
            }
            Message::WindowEventOccured(event) => {
                if let Event::Window(id, window::Event::CloseRequested) = event {
                    window::close(id)
                } else {
                    Command::none()
                }
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        Container::new(
            column![
                Text::new("日本語サンプル。上手く出ているだろうか").font(Font {
                    family: Family::Name("IBM Plex Sans JP"),
                    weight: Weight::Bold,
                    ..Font::default()
                }),
                vertical_space(),
                Button::new(
                    Text::new("+")
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center)
                        .size(40)
                )
                .on_press(Message::IncrementPressed)
                .width(Length::Fill)
                .padding(50),
                Text::new(self.value.to_string())
                    .width(Length::Fill)
                    .horizontal_alignment(Horizontal::Center)
                    .size(40),
                vertical_space(),
            ]
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .padding(50)
        .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        // ウィンドウが閉じられたときに`Message::WindowClosed`を発行する
        /*event::listen().map(|event| match event {
            window::Event::Closed => Some(Message::WindowClosed),
            _ => None,
        })*/
        event::listen().map(Message::WindowEventOccured)
    }
}

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
    let state = State {
        value: last_value, // 前回の値を引き継ぐ
        increment_button: button::State::new(),
    };
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
    State::run(settings);
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
