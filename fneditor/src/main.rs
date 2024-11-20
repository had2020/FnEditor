use dioxus::prelude::*;
//use dioxus_logger::tracing::Level; ???
//use nfd::{open_file_dialog, Response}; // imports file dialog windows for rust cross platform

//use std::fs::File;
//use std::io::Write;

//use std::io::Read;

use web_sys::*;
use web_sys::FileReader;


fn save_file(file_name: &str, content: &str) {
    let window = web_sys::window().unwrap();
    let blob = Blob::new_with_str(content).unwrap();
    let url = Url::create_object_url_with_blob(&blob).unwrap();
    window.open_with_url_and_target(&url, "_blank").unwrap();
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

    let mut file_data = use_signal(|| "".to_string());

    rsx! {
        link { rel: "stylesheet", href: "styles.css" } // styling link

        div {
            class: "container",

            p {}

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
                            for file in files {
                                let file_reader = FileReader::new().unwrap();
                                let file_content = file_reader.as_string().unwrap();
                                file_data.set(file_content)
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

