use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::prelude::*;
use nannou::rand::Rng;

fn main() {
    nannou::sketch(view).size(1000, 1000).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        let win = app.window_rect();
        draw.background().color(BEIGE);
        draw_square(&draw, pt2(0.0, 0.0), win.w() * 0.95, 4);
    }
    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::S) {
        let path = format!("output/{}/image.png", app.exe_name().unwrap());
        app.main_window().capture_frame(&path);
        println!("file: {} saved!", path);
    }
}

fn draw_square(draw: &Draw, loc: Point2, size: f32, depth: usize) {
    if (random_f32() > 0.7 && depth < 4) || (depth == 0) {
        fill_square(draw, loc, size * 0.95, choose_color());
    } else {
        let d = size * 0.5;
        draw_square(draw, pt2(loc.x + d * 0.5, loc.y + d * 0.5), d, depth - 1);
        draw_square(draw, pt2(loc.x - d * 0.5, loc.y + d * 0.5), d, depth - 1);
        draw_square(draw, pt2(loc.x + d * 0.5, loc.y - d * 0.5), d, depth - 1);
        draw_square(draw, pt2(loc.x - d * 0.5, loc.y - d * 0.5), d, depth - 1);
    }
}

fn draw_line(draw: &Draw, start: Point2, end: Point2, loc: Point2, color: Rgb<Srgb, u8>) {
    let distance = start.distance(end);
    let n = ((distance / 20.0).ceil() as i32).max(2);
    let points = Vec::from_iter((0..n).map(|i| {
        let x = map_range(i, 0, n - 1, start.x, end.x) + random_range(-1.0, 1.0) * 3.0;
        let y = map_range(i, 0, n - 1, start.y, end.y) + random_range(-1.0, 1.0) * 3.0;
        pt2(x, y)
    }));
    draw.xy(loc)
        .polyline()
        .weight(5.0)
        .points(points)
        .color(color);
}

fn fill_square(draw: &Draw, loc: Point2, size: f32, color: Rgb<Srgb, u8>) {
    let tl = pt2(-size * 0.5, size * 0.5);
    let tr = pt2(size * 0.5, size * 0.5);
    let bl = pt2(-size * 0.5, -size * 0.5);
    let br = pt2(size * 0.5, -size * 0.5);
    let n = 5 as i32;
    let points = Vec::from_iter((1..4 * n).map(|i| {
        let p = if (i % 2 == 0) && (i < 2 * n - 1) {
            let x = map_range(i, 0, 2 * n, tl.x, bl.x);
            let y = map_range(i, 0, 2 * n, tl.y, bl.y);
            pt2(x, y)
        } else if (i % 2 == 1) && (i <= 2 * n - 1) {
            let x = map_range(i, -1, 2 * n - 1, tl.x, tr.x);
            let y = map_range(i, -1, 2 * n - 1, tl.y, tr.y);
            pt2(x, y)
        } else if (i % 2 == 0) && (i > 2 * n - 1) {
            let x = map_range(i, 2 * n, 4 * n, bl.x, br.x);
            let y = map_range(i, 2 * n, 4 * n, bl.y, br.y);
            pt2(x, y)
        } else {
            let x = map_range(i, 2 * n - 1, 4 * n - 1, tr.x, br.x);
            let y = map_range(i, 2 * n - 1, 4 * n - 1, tr.y, br.y);
            pt2(x, y)
        };
        p
    }));
    let mut s = tl;
    for p in points.iter() {
        draw_line(draw, s, *p, loc, color);
        s = *p;
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
