use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(1000, 1000).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let t = app.time * 10.0;
    draw.background().color(WHITE);
    let n = 25;
    let wd = win.w() / n as f32;
    for i in 0..n {
        for j in 0..n {
            let perlin = Perlin::new();
            let x = wd * 0.5 + wd * i as f32 - win.w() * 0.5;
            let y = wd * 0.5 + wd * j as f32 - win.h() * 0.5;
            let r = perlin.get([x as f64, (y - t) as f64]);
            if r > 0.3 {
                draw_shape(&draw, pt2(x, y), wd, 0.0);
            } else {
                draw_shape(&draw, pt2(x, y), wd, 0.5 * f32::PI());
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn draw_shape(draw: &Draw, loc: Point2, size: f32, rot: f32) {
    let mut rt = Vec::from_iter((0..=90).map(|i| {
        let theta = i as f32 * f32::PI() / 180.0 + f32::PI();
        let x = size * 0.5 + size * 0.5 * theta.cos();
        let y = size * 0.5 + size * 0.5 * theta.sin();
        pt2(x, y)
    }));
    rt.push(pt2(size * 0.5, size * 0.5));
    let mut ld = Vec::from_iter((0..=90).map(|i| {
        let theta = i as f32 * f32::PI() / 180.0;
        let x = -size * 0.5 + size * 0.5 * theta.cos();
        let y = -size * 0.5 + size * 0.5 * theta.sin();
        pt2(x, y)
    }));
    ld.push(pt2(-size * 0.5, -size * 0.5));
    draw.x_y(loc.x, loc.y)
        .rotate(rot)
        .polygon()
        .points(rt)
        .color(GREEN);
    draw.x_y(loc.x, loc.y)
        .rotate(rot)
        .polygon()
        .points(ld)
        .color(GREEN);
}
