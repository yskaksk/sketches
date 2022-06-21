use nannou::prelude::*;

use nannou::noise::{NoiseFn, Perlin};

fn main() {
    nannou::sketch(view).size(1000, 1000).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let perlin = Perlin::new();
    let t = app.time / 10.0;

    draw.background().color(WHITE);

    //let points = (0..100).map(|i| {
    //    let x = map_range(i, 0, 99, win.left(), win.right());
    //    let y1 = 128.0 * perlin.get([(i as f64 / 100.0), t as f64]) as f32;
    //    let y2 = 64.0 * perlin.get([(2.0 * i as f64 / 100.0), t as f64]) as f32;
    //    let y3 = 32.0 * perlin.get([4.0 * i as f64, t as f64]) as f32;
    //    let y4 = 16.0 * perlin.get([8.0 * i as f64, t as f64]) as f32;
    //    let y5 = 8.0 * perlin.get([16.0 * i as f64, t as f64]) as f32;
    //    let y6 = 4.0 * perlin.get([32.0 * i as f64, t as f64]) as f32;
    //    pt2(x, y1 + y2 + y3 + y4 + y5 + y6)
    //});
    //draw.polyline().points(points);
    let n = 100;
    let d = win.w() / n as f32;
    let mut total = vec![];
    for i in 0..n {
        for j in 0..n {
            let x = d * 0.5 + d * i as f32 - win.w() * 0.5;
            let y = d * 0.5 + d * j as f32 - win.h() * 0.5;
            let a = (1.0 + perlin.get([x as f64, y as f64 + t as f64]) as f32) / 2.0;
            total.push(a);
            draw.x_y(x, y).rect().w_h(d, d).color(rgb(a, a, a));
        }
    }
    let ave: f32 = total.iter().sum::<f32>() / (n * n) as f32;
    let std: f32 = total.iter().fold(0.0, |std, a| (a - ave).powf(2.0) + std) / (n * n) as f32;
    println!("ave: {}, std: {}", ave, std);

    draw.to_frame(app, &frame).unwrap();
}
