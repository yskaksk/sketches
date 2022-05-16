use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(600, 600).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let win = app.window_rect();
    let n = 45;
    let d = win.h() / 2.0;

    let theta = 2.0 * f32::PI() / (n as f32);
    let orig = Point2::new(0.0, 0.0);
    for i in 0..n {
        let a = Point2::new(d * (theta * i as f32).cos(), d * (theta * i as f32).sin());
        let b = Point2::new(
            d * (theta * (i + 1) as f32).cos(),
            d * (theta * (i + 1) as f32).sin(),
        );
        draw.tri().points(orig, a, b).color(hsv(
            map_range(theta * i as f32, 0.0, 2.0 * f32::PI(), 0.0, 1.0),
            map_range(app.mouse.x, win.left(), win.right(), 0.0, 1.0),
            map_range(app.mouse.y, win.bottom(), win.top(), 0.0, 1.0),
        ));
    }
    draw.to_frame(app, &frame).unwrap();
}
