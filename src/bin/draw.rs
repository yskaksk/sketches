use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(1000, 1000).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let width = 200.0;
    let t = app.time;
    draw.background().color(WHITE);
    draw.x_y(win.w() * 0.5 * t.sin(), 0.0)
        .quad()
        .points(
            pt2(-width / 2.0, win.top()),
            pt2(width / 2.0, win.top()),
            pt2(width / 2.0, win.bottom()),
            pt2(-width / 2.0, win.bottom()),
        )
        .color(BLUE);
    draw.to_frame(app, &frame).unwrap();
}
