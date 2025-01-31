use laststraw::*;

use rfd::FileDialog;
use std::path::PathBuf;
fn open_dialog() -> String {
    let file: Option<PathBuf> = FileDialog::new()
        .add_filter("text", &["txt", "rs", "rtf"])
        .add_filter("rust", &["rs", "toml"])
        .set_directory("/")
        .pick_file();

    match file {
        Some(path) => path.to_string_lossy().to_string(),
        None => String::new(),
    }
}

fn main() {
    let mut app = App::new(500, 500, "test");

    let mut text_position_offset: f32 = 0.0;
    let mut file_path: String = String::new();

    asx!({
        set_window_color(&mut app, "Obsidian");

        set_next_button(&mut app, position!(30.0, 30.0, 30.0)); // maybe wrap as struct
        set_next_button_text(&mut app, "Open");
        button!({
            file_path = open_dialog();
        });

        if input_pressed(&app, "esc") {
            app.should_close = true;
        }

        // basic scrolling
        if input_pressed(&app, "up") {
            text_position_offset -= 5.0;
        }

        if input_pressed(&app, "down") {
            text_position_offset += 5.0;
        }

        let new_y_pos: f32 = 85.0 + text_position_offset;
        let editable_position: Position = position!(10.0, new_y_pos, 30.0);
        let texty = editable_lines(&mut app, editable_position, "text:", "White", false);

        limit_fps(&mut app, 60.0);
    });

    println!("app closed after window code.");
}
