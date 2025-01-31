use laststraw::*;

use std::fs::File;
use std::io::BufRead;

use std::time::*;

use rfd::FileDialog;
use std::path::PathBuf;
fn open_dialog() -> String {
    let file: Option<PathBuf> = FileDialog::new()
        .add_filter("text", &["txt", "rs"])
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
    let mut loaded: bool = false;
    let mut now = Instant::now();
    let mut current_data = vec![String::new(), String::new()];

    asx!({
        set_window_color(&mut app, "Obsidian");

        set_next_button(&mut app, position!(1.0, 20.0, 30.0));
        set_next_button_text(&mut app, "Open");
        button!({
            if !loaded {
                loaded = true;
                file_path = open_dialog();

                let file = File::open(&file_path).unwrap();
                let reader = std::io::BufReader::new(file);

                let mut line_interation = 0;
                for line in reader.lines() {
                    //println!("{}", line.unwrap());
                    line_interation += 1;
                    current_data[line_interation - 1] = line.unwrap();
                }

                now = Instant::now();
            }
            if now.elapsed() > Duration::new(0, 1) {
                loaded = false;
            }
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
        app.multi_line_storing[1][0] = "hello".to_string();
        let _texty = editable_lines(&mut app, editable_position, "text:", "White", false);
        limit_fps(&mut app, 60.0);
    });

    println!("app closed after window code.");
}
