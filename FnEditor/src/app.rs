#![allow(non_snake_case)]

use dioxus::prelude::*;
//use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub fn App() -> Element {

    //let mut filename = use_signal(|| String::new());
    let mut file_content = use_signal(|| String::new());

    rsx! {
        link { rel: "stylesheet", href: "styles.css" }
        main {
            class: "container",

            div { class: "topnav",
                input {
                    class: "verticalLine",
                    r#type: "file",
                    // pick multiple files
                    multiple: false,
                    onchange: move |evt| {
                        if let Some(file_engine) = &evt.files() {
                            let files = file_engine.files();
                            for file_name in files {
                                let content = std::fs::read_to_string(file_name).expect("Failed to read file");
                                file_content.set(content); // Use .set() to update the signal
                                //file_content = std::fs::read_to_string(file_name).unwrap();
                                //filename.set(file_name);
                            }
                        }
                    }
                    
                }
                button { r#type: "submit", "Save File"}
                button { r#type: "submit", "New File"}
            }

            p { "{file_content}" }

            div { 
                class: "textual_area",
                contenteditable: true,
                p {
                    id: "text_content",
                    class: "text_content",
                    "{file_content}",
                }
            }
        }
    }
}
