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
// アプリケーションの状態を定義する構造体
pub struct State {
    // 数値を保持するフィールド
    pub value: i32,
}

// メッセージを定義する列挙体
#[derive(Debug, Clone)]
pub enum Message {
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
        (State { value: 0 }, Command::none())
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
        Subscription::batch(vec![event::listen().map(Message::WindowEventOccured)])
    }
}
