use laststraw::*;

fn main() {
    let mut app = App::new(500, 500, "test");

    asx!({
        set_window_color(&mut app, "Obsidian"); // is layored so this is back

        if input_pressed(&app, "esc") {
            app.should_close = true;
        }

        set_next_button(&mut app, position!(30.0, 30.0, 30.0)); // maybe wrap as struct
        set_next_button_text(&mut app, "SAVE");
        button!({
            println!("save");
        });

        set_next_button(&mut app, position!(100.0, 30.0, 30.0)); // maybe wrap as struct
        set_next_button_text(&mut app, "LOAD");
        button!({
            println!("load");
        });

        let texty = editable_lines(
            &mut app,
            position!(100.0, 50.0, 50.0),
            "enter:",
            "Blue",
            false,
        );
        single_line_text(&mut app, position!(0.0, 30.0, 40.0), &texty); // you can acess the value later, will be empty, never recived input

        limit_fps(&mut app, 60.0);
    });

    println!("app closed after window code.");
}
