use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(600, 600).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    let win = app.window_rect();
    let mouse_y = app.mouse.y;
    let background_color = hsv(
        map_range(mouse_y, win.left(), win.right(), 0.0, 1.0),
        1.0,
        1.0,
    );
    let center_color = hsv(
        map_range(mouse_y / 2.0, win.left(), win.right(), 0.0, 1.0),
        1.0,
        1.0,
    );
    draw.background().color(background_color);

    let d = app.mouse.x.abs() * 1.9;
    draw.rect().x_y(0.0, 0.0).w_h(d, d).color(center_color);

    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::P) {
        println!("back color: {:?}", background_color);
        println!("center color: {:?}", center_color);
    }
}
