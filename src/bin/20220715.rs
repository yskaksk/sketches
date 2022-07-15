use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(1000, 1000).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BEIGE);
    let win = app.window_rect();
    let max_size = win.w() * 0.9;
    let min_size = 10.0;
    let time = app.time * 4.0;
    let n = 30;
    let t = time % (n as f32);
    for i in 0..=n {
        let size = map_range(i, 0, n, min_size, max_size);
        let d = size * 0.5;
        let w = if t > i as f32 { 7.0 } else { 3.0 };
        draw_line(&draw, pt2(d, d), pt2(d, -d), 3.0, pt2(0.0, 0.0), w);
        draw_line(&draw, pt2(d, -d), pt2(-d, -d), 3.0, pt2(0.0, 0.0), w);
        draw_line(&draw, pt2(-d, -d), pt2(-d, d), 3.0, pt2(0.0, 0.0), w);
        draw_line(&draw, pt2(-d, d), pt2(d, d), 3.0, pt2(0.0, 0.0), w);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn draw_line(draw: &Draw, start: Point2, end: Point2, weight: f32, loc: Point2, d: f32) {
    let distance = start.distance(end);
    let n = ((distance / 20.0).ceil() as i32).max(2);
    let points = Vec::from_iter((0..n).map(|i| {
        let x = map_range(i, 0, n - 1, start.x, end.x) + random_range(-1.0, 1.0) * d;
        let y = map_range(i, 0, n - 1, start.y, end.y) + random_range(-1.0, 1.0) * d;
        pt2(x, y)
    }));
    draw.xy(loc)
        .polyline()
        .weight(weight)
        .points(points)
        .color(rgb(0.2, 0.2, 0.2));
}
