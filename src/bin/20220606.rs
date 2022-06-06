use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(1000, 1000).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(WHITE);
        let win = app.window_rect();
        draw_shape(&draw, pt2(0.0, 0.0), win.w(), rgb(0.0, 0.0, 0.0), 5);
        draw_shape(&draw, pt2(0.0, 0.0), win.w(), rgb(0.2, 0.2, 0.2), 4);
        draw_shape(&draw, pt2(0.0, 0.0), win.w(), rgb(0.4, 0.4, 0.4), 3);
        draw_shape(&draw, pt2(0.0, 0.0), win.w(), rgb(0.6, 0.6, 0.6), 2);
        draw_shape(&draw, pt2(0.0, 0.0), win.w(), rgb(0.8, 0.8, 0.8), 1);
    }
    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::S) {
        let path = format!("output/{}/image.png", app.exe_name().unwrap());
        app.main_window().capture_frame(&path);
        println!("file: {} saved!", path);
    }
}

fn draw_shape(draw: &Draw, center: Point2, size: f32, color: Rgb<f32>, depth: usize) {
    if depth == 0 {
        draw_square(draw, center, size, color);
    } else {
        let d = size / 6.0;
        draw_shape(
            draw,
            pt2(center.x + d, center.y + d),
            4.0 * d,
            color,
            depth - 1,
        );
        draw_shape(
            draw,
            pt2(center.x - 2.0 * d, center.y + 2.0 * d),
            2.0 * d,
            color,
            depth - 1,
        );
        draw_shape(
            draw,
            pt2(center.x - 2.0 * d, center.y),
            2.0 * d,
            color,
            depth - 1,
        );
        draw_shape(
            draw,
            pt2(center.x - 2.0 * d, center.y - 2.0 * d),
            2.0 * d,
            color,
            depth - 1,
        );
        draw_shape(
            draw,
            pt2(center.x, center.y - 2.0 * d),
            2.0 * d,
            color,
            depth - 1,
        );
        draw_shape(
            draw,
            pt2(center.x + 2.0 * d, center.y - 2.0 * d),
            2.0 * d,
            color,
            depth - 1,
        );
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
