use iced::widget::Svg;
use iced::{Sandbox, Element, Settings, Length};

#[derive(Default)]
struct RenderSvg;

#[derive(Debug)]
enum Message {}

impl Sandbox for RenderSvg {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("RenderSvg")
    }

    fn update(&mut self, _message: Message) {}

    fn view(&self) -> Element<Message> {
        Svg::from_path(String::from("svg/ace_of_spades.svg")).height(Length::Fill).into()
    }
}

fn main() -> iced::Result {
    RenderSvg::run(Settings::default())
}