use itertools::Itertools;
use nannou::prelude::*;

fn main() {
    nannou::app(model).view(view).update(update).run()
}

const N: usize = 120;

struct Model {
    squares: Vec<Vec<Point2>>,
}

fn model(app: &App) -> Model {
    let d = 720;
    app.new_window().size(d, d).build().unwrap();
    let win = app.window_rect();
    let delta = d as f32 / 64.0;
    let squares = (0..4).rev().cartesian_product(0..4).map(|(i, j)| {
        let x = (d as f32 / 4.0) * j as f32 + d as f32 / 8.0 - win.w() / 2.0
            + random_range(-delta, delta);
        let y = (d as f32 / 4.0) * i as f32 + d as f32 / 8.0 - win.h() / 2.0
            + random_range(-delta, delta);
        Vec::from_iter(square_points(pt2(x, y), win.w() / 8.0, N, 0.0))
    });
    Model {
        squares: Vec::from_iter(squares),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(WHITE);
    }
    if frame.nth() < 300 {
        for s in model.squares.iter() {
            let c = rgba(0.6, 0.6, 0.6, 0.01);
            draw.polyline().weight(3.0).points(s.clone()).color(c);
        }
    }
    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::S) {
        let path = format!("output/{}/image.png", app.exe_name().unwrap());
        app.main_window().capture_frame(&path);
        println!("file: {} saved!", path);
    }
}

fn update(_: &App, model: &mut Model, _: Update) {
    let squares = model.squares.iter().enumerate().map(|(i, s)| {
        let d = map_range(i, 0, 15, 0.2, 1.3);
        let points = s.iter().map(|p| {
            let x = p.x + random_range(-d, d);
            let y = p.y + random_range(-d, d);
            pt2(x, y)
        });
        Vec::from_iter(points)
    });
    model.squares = Vec::from_iter(squares);
}

fn square_points(center: Point2, diameter: f32, npoints: usize, round: f32) -> Vec<Point2> {
    let points = (0..=npoints).map(|i| {
        let p = if i < npoints / 4 {
            let x = map_range(i, 0, npoints / 4, diameter, 0.0);
            let y = diameter - x;
            pt2(x, y)
        } else if i < (2 * (npoints / 4)) {
            let x = map_range(i, npoints / 4, 2 * npoints / 4, 0.0, -diameter);
            let y = x + diameter;
            pt2(x, y)
        } else if i < (3 * (npoints / 4)) {
            let x = map_range(i, 2 * npoints / 4, 3 * npoints / 4, -diameter, 0.0);
            let y = -x - diameter;
            pt2(x, y)
        } else {
            let x = map_range(i, 3 * npoints / 4, npoints, 0.0, diameter);
            let y = x - diameter;
            pt2(x, y)
        };
        let theta = f32::PI() / 4.0 + round;
        pt2(
            center.x + p.x * theta.cos() - p.y * theta.sin(),
            center.y + p.x * theta.sin() + p.y * theta.cos(),
        )
    });
    return Vec::from_iter(points);
}
