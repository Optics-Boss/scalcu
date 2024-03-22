use iced::widget::{button, column, text, row, container};
use iced::{Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    input: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Temperature {
    Fahrenheit,
    Celsius,
    Kelvin,
    Other,
}

impl Temperature {
    const ALL: [Temperature; 4] = [
        Temperature::Fahrenheit,
        Temperature::Celsius,
        Temperature::Kelvin,
        Temperature::Other,
    ];
}

impl std::fmt::Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Temperature::Fahrenheit => "Fahrenheit",
                Temperature::Celsius => "Celsius",
                Temperature::Kelvin => "Kelvin",
                Temperature::Other => "Other",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeTextInput(String),
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { 
            input: "".into(), 
        }
    }

    fn title(&self) -> String {
        String::from("Scalcu - Calculator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeTextInput(value) => {
                self.input = value.into();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        
        let header = container(
            row![
                 text(&self.input)
            ]
            .align_items(Alignment::Center)
        );

        let left =  container (column![
                button("0").on_press(Message::ChangeTextInput("0".to_string())),
                button("1").on_press(Message::ChangeTextInput("1".to_string())),
                button("2").on_press(Message::ChangeTextInput("2".to_string())),
                button("3").on_press(Message::ChangeTextInput("3".to_string())),
            ]
            .padding(20)
            .align_items(Alignment::Center)
        );

        let middle = container (column![
                button("4").on_press(Message::ChangeTextInput("0".to_string())),
                button("5").on_press(Message::ChangeTextInput("1".to_string())),
                button("6").on_press(Message::ChangeTextInput("2".to_string())),
            ]
            .padding(20)
            .align_items(Alignment::Center)
        );

        let right = container (column![
                button("7").on_press(Message::ChangeTextInput("1".to_string())),
                button("8").on_press(Message::ChangeTextInput("2".to_string())),
                button("9").on_press(Message::ChangeTextInput("3".to_string())),
            ]
            .padding(20)
            .align_items(Alignment::Center)
        );

        column![header, row![left, middle, right]].into()
    }

}
