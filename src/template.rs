fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    // do something
    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::S) {
        let path = format!("output/{}/image.png", app.exe_name().unwrap());
        app.main_window().capture_frame(&path);
        println!("file: {} saved!", path);
    }
    // gif
    if frame.nth() < 300 {
        let path = gif_save_path(app, frame.nth() as usize);
        app.main_window().capture_frame(&path);
    }
}

fn choose_color() -> Rgb<Srgb, u8> {
    let colors = [
        DARKRED,
        CADETBLUE,
        DARKBLUE,
        DARKOLIVEGREEN,
        DODGERBLUE,
        LIGHTSALMON,
        TOMATO,
    ];
    colors[nannou::rand::thread_rng().gen_range(0..colors.len())]
}
