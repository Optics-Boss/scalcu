#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus::launch(App);
}

pub enum Arithmetic{
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Other,
}

#[component]
fn App() -> Element {
    let mut result_1 = use_signal(|| "".to_string());

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        h1 { class: "white", "{result_1}"}
        div { id: "numbers__left",
            button { onclick: move |_| result_1.write().push('7'), "7"}
            button { onclick: move |_| result_1.write().push('4'), "4"}
            button { onclick: move |_| result_1.write().push('1'), "1"}
            button { onclick: move |_| result_1.write().push('0'), "0"}
        }

        div { id: "numbers__center",
            button { onclick: move |_| result_1.write().push('8'), "8"}
            button { onclick: move |_| result_1.write().push('5'), "5"}
            button { onclick: move |_| result_1.write().push('2'), "2"}
        }

        div { id: "numbers__right",
            button { onclick: move |_| result_1.write().push('9'), "9"}
            button { onclick: move |_| result_1.write().push('6'), "6"}
            button { onclick: move |_| result_1.write().push('3'), "3"}
        }

        div { id: "other",
            button { onclick: move |_| result_1.set("".to_string()), "C"}
        }
    }
}
