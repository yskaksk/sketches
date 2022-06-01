use nannou::prelude::*;

use sketches::util;

fn main() {
    nannou::sketch(view).size(1000, 1000).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let w = win.w();
    draw.background().color(WHITE);

    let n = 10;
    let size = w / (n - 1) as f32;
    let half = size / 2.0;
    for i in 0..n {
        for j in 0..n {
            let mut loc = util::map_range_win(win, i, j, n);
            if j % 2 == 0 {
                loc.x -= half;
            }
            draw_shape(&draw, size, loc);
        }
    }

    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::S) {
        let path = format!("output/{}/image.png", app.exe_name().unwrap());
        app.main_window().capture_frame(&path);
        println!("file: {} saved!", path);
    }
}

fn draw_arc(draw: &Draw, center: Point2, r: f32, theta_from: f32, theta_to: f32) {
    let points = (0..100).map(|i| {
        let theta = map_range(i, 0, 99, theta_from, theta_to);
        let x = center.x + r * theta.cos();
        let y = center.y + r * theta.sin();
        pt2(x, y)
    });
    draw.polyline().weight(3.0).points(points).color(BLACK);
}

fn draw_shape(draw: &Draw, size: f32, loc: Point2) {
    let d = size / 2.0;
    let sq2 = 2.0.sqrt();
    draw_arc(
        &draw,
        pt2(d, 2.0 * d) + loc,
        d * sq2,
        1.25 * f32::PI(),
        1.5 * f32::PI(),
    );
    draw_arc(
        &draw,
        pt2(-d, 2.0 * d) + loc,
        d * sq2,
        1.5 * f32::PI(),
        1.75 * f32::PI(),
    );
    draw_arc(&draw, loc, d * sq2, 1.25 * f32::PI(), 1.75 * f32::PI());

    let d2 = d * (3.0 - sq2) / 2.0;
    draw_arc(
        &draw,
        pt2(d, d2 - d) + loc,
        d2,
        0.5 * f32::PI(),
        1.5 * f32::PI(),
    );
    draw_arc(
        &draw,
        pt2(-d, d2 - d) + loc,
        d2,
        0.5 * f32::PI(),
        1.5 * f32::PI(),
    );
}
