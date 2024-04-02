#![allow(non_snake_case)]

use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut result_1 = use_signal(|| "".to_string());

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        h1 { class: "white", "{result_1}"}
        div { id: "numbers__left",
            button { onclick: move |_| result_1.set("7".to_string()), "7"}
            button { onclick: move |_| result_1.set("4".to_string()), "4"}
            button { onclick: move |_| result_1.set("1".to_string()), "1"}
            button { onclick: move |_| result_1.set("0".to_string()), "0"}
        }

        div { id: "numbers__center",
            button { onclick: move |_| result_1.set("8".to_string()), "8"}
            button { onclick: move |_| result_1.set("5".to_string()), "5"}
            button { onclick: move |_| result_1.set("2".to_string()), "2"}
        }

        div { id: "numbers__right",
            button { onclick: move |_| result_1.set("9".to_string()), "9"}
            button { onclick: move |_| result_1.set("6".to_string()), "6"}
            button { onclick: move |_| result_1.set("3".to_string()), "3"}
        }
    }
}
