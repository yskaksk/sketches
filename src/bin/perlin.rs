use nannou::prelude::*;

use nannou::noise::{NoiseFn, Perlin};

fn main() {
    nannou::sketch(view).size(720, 720).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let perlin = Perlin::new();
    let t = app.time / 10.0;

    draw.background().color(WHITE);

    let points = (0..100).map(|i| {
        let x = map_range(i, 0, 99, win.left(), win.right());
        let y1 = 128.0 * perlin.get([(i as f64 / 100.0), t as f64]) as f32;
        let y2 = 64.0 * perlin.get([(2.0 * i as f64 / 100.0), t as f64]) as f32;
        let y3 = 32.0 * perlin.get([4.0 * i as f64, t as f64]) as f32;
        let y4 = 16.0 * perlin.get([8.0 * i as f64, t as f64]) as f32;
        let y5 = 8.0 * perlin.get([16.0 * i as f64, t as f64]) as f32;
        let y6 = 4.0 * perlin.get([32.0 * i as f64, t as f64]) as f32;
        pt2(x, y1 + y2 + y3 + y4 + y5 + y6)
    });
    draw.polyline().points(points);

    draw.to_frame(app, &frame).unwrap();
}
