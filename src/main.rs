use iced::widget::{button, column, text, row, container};
use iced::{Alignment, Element, Sandbox, Settings};

pub fn main() -> iced::Result {
    Counter::run(Settings::default())
}

struct Counter {
    input: String,
    arithmetic: Arithmetic,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Arithmetic {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Other,
}

impl Arithmetic {
    const ALL: [Arithmetic; 5] = [
        Arithmetic::Addition,
        Arithmetic::Subtraction,
        Arithmetic::Multiplication,
        Arithmetic::Division,
        Arithmetic::Other,
    ];
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeTextInput(String),
    SetArithmetic(Arithmetic),
    Clear(),
}

impl Sandbox for Counter {
    type Message = Message;

    fn new() -> Self {
        Self { 
            input: "".to_owned(), 
            arithmetic: Arithmetic::Other,
        }
    }

    fn title(&self) -> String {
        String::from("Scalcu - Calculator")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ChangeTextInput(value) => {
                self.input += &value;
            },
            Message::SetArithmetic(arithmetic) => {
                self.arithmetic = arithmetic;
            }
            Message::Clear() => {
                self.input = "".to_string();
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

        let settings = container(
            row![
                button("C").on_press(Message::Clear()),
                button("CE").on_press(Message::Clear()),
                button("%").on_press(Message::SetArithmetic(Arithmetic::Division)),
                button("X").on_press(Message::SetArithmetic(Arithmetic::Multiplication)),
                button("+").on_press(Message::SetArithmetic(Arithmetic::Addition)),
                button("-").on_press(Message::SetArithmetic(Arithmetic::Subtraction)),
                button("=").on_press(Message::ChangeTextInput("1".to_string())),
            ]
        );

        let left = container(
            column![
                button("0").on_press(Message::ChangeTextInput("0".to_string())),
                button("1").on_press(Message::ChangeTextInput("1".to_string())),
                button("2").on_press(Message::ChangeTextInput("2".to_string())),
                button("3").on_press(Message::ChangeTextInput("3".to_string())),
            ]
            .padding(20)
            .align_items(Alignment::Center)
        );

        let middle = container (column![
                button("4").on_press(Message::ChangeTextInput("4".to_string())),
                button("5").on_press(Message::ChangeTextInput("5".to_string())),
                button("6").on_press(Message::ChangeTextInput("6".to_string())),
            ]
            .padding(20)
            .align_items(Alignment::Center)
        );

        let right = container (column![
                button("7").on_press(Message::ChangeTextInput("7".to_string())),
                button("8").on_press(Message::ChangeTextInput("8".to_string())),
                button("9").on_press(Message::ChangeTextInput("9".to_string())),
            ]
            .padding(20)
            .align_items(Alignment::Center)
        );


        column![header, settings, row![left, middle, right]].into()
    }

}
