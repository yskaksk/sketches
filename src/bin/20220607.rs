use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(1000, 1000).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(WHITE);
    let x = (win.w() / 3.0) * (7.0 * app.time * f32::PI() / 10.0).cos();
    let y = (win.h() / 3.0) * (3.0 * app.time * f32::PI() / 10.0).sin();
    draw.ellipse().w_h(200.0, 200.0).x_y(x, y).color(RED);
    let t = (app.time * 400.0) % win.h();
    draw_shape(&draw, win.w(), rgb(0.4, 0.4, 0.4), 4, t);
    let t = (app.time * 200.0) % win.h();
    draw_shape(&draw, win.w(), rgb(0.2, 0.2, 0.2), 2, t);

    draw.to_frame(app, &frame).unwrap();
}

fn draw_shape(draw: &Draw, size: f32, color: Rgb<f32>, n: usize, t: f32) {
    let d = size / n as f32;
    for i in 0..n {
        if i % 2 == 0 {
            for j in 0..(2 * n) {
                let x = -size / 2.0 + d / 2.0 + i as f32 * d;
                let y = t + size / 2.0 - d / 2.0 - j as f32 * d;
                draw_square(&draw, pt2(x, y), d, color);
            }
        } else {
            for j in 0..(2 * n) {
                let x = -size / 2.0 + d / 2.0 + i as f32 * d;
                let y = -t - size / 2.0 + d / 2.0 + j as f32 * d;
                draw_square(&draw, pt2(x, y), d, color);
            }
        }
    }
}

fn draw_square(draw: &Draw, loc: Point2, size: f32, color: Rgb<f32>) {
    let r = 0.95 * size;
    let mut points = Vec::from_iter((0..=180).map(|i| {
        let theta = i as f32 * (f32::PI() / 180.0);
        let x = 0.5 * r * theta.cos();
        let y = 0.5 * r * theta.sin();
        pt2(x, y)
    }));
    points.push(pt2(-size / 2.0, 0.0));
    points.push(pt2(-size / 2.0, size / 2.0));
    points.push(pt2(size / 2.0, size / 2.0));
    points.push(pt2(size / 2.0, 0.0));
    draw.x_y(loc.x, loc.y).polygon().points(points).color(color);

    let mut points = Vec::from_iter((0..=180).map(|i| {
        let theta = i as f32 * (f32::PI() / 180.0) + f32::PI();
        let x = 0.5 * r * theta.cos();
        let y = 0.5 * r * theta.sin();
        pt2(x, y)
    }));
    points.push(pt2(size / 2.0, 0.0));
    points.push(pt2(size / 2.0, -size / 2.0));
    points.push(pt2(-size / 2.0, -size / 2.0));
    points.push(pt2(-size / 2.0, 0.0));
    draw.x_y(loc.x, loc.y).polygon().points(points).color(color);
}
