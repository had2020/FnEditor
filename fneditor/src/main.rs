use laststraw::*;

fn main() {
    let mut app = App::new(500, 500, "test");

    let mut text_position_offset: f32 = 0.0;
    let mut file_path: String = String::new();

    asx!({
        set_window_color(&mut app, "Obsidian");

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

        let path = editable_lines(
            &mut app,
            position!(10.0, 30.0, 30.0),
            "path:",
            "White",
            true,
        );

        limit_fps(&mut app, 60.0);
    });

    println!("app closed after window code.");
}
