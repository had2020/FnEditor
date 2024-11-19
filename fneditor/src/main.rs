use dioxus::prelude::*;
//use dioxus_logger::tracing::Level; ???
//use nfd::{open_file_dialog, Response}; // imports file dialog windows for rust cross platform

//use std::fs::File;
//use std::io::Write;

use std::io::Read;
use web_sys::FileReader;


fn save_file() {
    log::info!("save");
    //let mut file = File::create("example.txt").expect("Unable to create file");
    //file.write_all(b"Hello, World!\n").expect("Unable to write to file");
}

fn open_file() {
    log::info!("save");
}

fn app() -> Element {
    log::info!("startup log");

    //let mut text_array: Vec<String> = vec! [String::new(); 126];
    let mut input_key = use_signal(|| "".to_string());
    let mut filename = use_signal(|| "".to_string());
    let mut written_text = use_signal(|| "".to_string());

    let mut filenames: Signal<Vec<String>> = use_signal(Vec::new);

    rsx! {
        link { rel: "stylesheet", href: "styles.css" } // styling link

        div {
            class: "container",

            div {
                class: "topnav",
    
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

                input {
                    // tell the input to pick a file
                    r#type: "file",
                    // list the accepted extensions
                    accept: ".txt,.rs",
                    // pick multiple files
                    multiple: false,
                    onchange: move |evt| {
                        if let Some(file_engine) = &evt.files() {
                            let files = file_engine.files();
                            for file_name in files {
                                filenames.write().push(file_name);
                            }
                        }
                    }
                }

                button {
                    onclick: move |_event| {
                        open_file();
                    },
                    "Open",
                }

            }

            textarea {
                
                oninput: move |event| {
                    written_text.set(event.value());
                },

                onkeydown: move |event| {
                    if event.key().to_string() == "Enter" {
                        input_key.set(event.key().to_string());
                        //text_array.push(written_line());
                        written_text.set(written_text() + "\n")
                    }
                },
                class: "textarea"
            }
        }
    }
}

fn main() {
    launch(app)
}

