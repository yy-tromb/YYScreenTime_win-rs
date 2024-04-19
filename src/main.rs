use iced::widget::{button, column, text};
use iced::{executor, window, Application, Command, Element, Event, Settings, Subscription, Theme};

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

// アプリケーションの状態を定義する構造体
struct State {
    // 数値を保持するフィールド
    value: i32,
    // ボタンの状態を保持するフィールド
    increment_button: button::State,
}

// メッセージを定義する列挙体
#[derive(Debug, Clone, Copy)]
enum Message {
    IncrementPressed,
    WindowClosed,
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
        }
    }

    fn view(&self) -> Element<Self::Message> {
        column::new()
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                    .on_press(Message::IncrementPressed),
            )
            .push(Text::new(self.value.to_string()))
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        // ウィンドウが閉じられたときに`Message::WindowClosed`を発行する
        //window::close_subscription().map(|_| Message::WindowClosed)
        event::listen().map(Message::EventOccurred)
    }
}

fn main() -> Result<()> {
    match windows_tools::enable_visual_styles() {
        Ok(_) => (),
        Err(ref errors) => match errors {
            windows_tools::EnableVStylesErrors::Win32(win32_error) => println!("{:?}", win32_error),
            windows_tools::EnableVStylesErrors::Standard(err) => println!("{:?}", err),
        },
    };
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
    loop {
        let state = State {
            value: last_value, // 前回の値を引き継ぐ
            increment_button: button::State::new(),
        };
        let mut settings = Settings::default();
        //settings.exit_on_close_request = false;
        State::run(settings);
    }
    Ok(())
}
