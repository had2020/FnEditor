use dioxus::prelude::*;
//use dioxus_logger::tracing::Level; ???
//use nfd::{open_file_dialog, Response}; // imports file dialog windows for rust cross platform

fn save_file() {
    log::info!("save");
}

fn open_file() {
    log::info!("save");
}

fn app() -> Element {
    log::info!("startup log");

    let mut array_strings = use_signal(|| Vec<String> = vec!)
    let mut writen_text = use_signal(|| "".to_string());
    let mut filename = use_signal(|| "".to_string());

    rsx! {
        link { rel: "stylesheet", href: "styles.css" } // styling link

        div {
            class: "container",

            div {
                class: "topnav",
            
                p {"text: {writen_text}"}

                button {
                    onclick: move |_event| {
                        open_file();
                    },
                    "Open",
                }
    
                button {
                    onclick: move |_event| {
                        save_file();
                    },
                    "Save",
                }
    
                input {
                    oninput: move |event| {
                        filename.set(event.value());
                    },
                    "filename",
                }
            }

            textarea {
                oninput: move |event| {
                    writen_text.set(event.value());
                },
                class: "textarea"
            }
        }
    }
}

fn main() {
    launch(app)
}

