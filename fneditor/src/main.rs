use dioxus::prelude::*;
//use dioxus_logger::tracing::Level; ???
//use nfd::{open_file_dialog, Response}; // imports file dialog windows for rust cross platform


fn app() -> Element {
    log::info!("startup log");

    let mut writen_text = use_signal(|| "");
    let mut name = use_signal(|| "bob".to_string());

    rsx! {
        link { rel: "stylesheet", href: "styles.css" } // styling link
        div {
            class:"container",

            p {"text: {writen_text}"},
            /* 
            button {
                "Open"
                oninput: move |_event| {
                }
            }
            button {
                "Save"
                oninput: move |_event| {
                }
            }
            */
            textarea {
                oninput: move |event| {
                    writen_text.set(event.value());
                },
                class:"textarea"
            }
        }
    }
}

fn main() {
    launch(app)
}

