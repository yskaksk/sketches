use nannou::prelude::*;
use nannou::rand;

use std::collections::VecDeque;

const STEP: usize = 10;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    points: VecDeque<Point2>,
    n: usize,
    f: usize,
}

fn random_point(app: &App) -> Point2 {
    let win = app.window_rect();
    let x = map_range(rand::random_f32(), 0.0, 1.0, win.left(), win.right());
    let y = map_range(rand::random_f32(), 0.0, 1.0, win.left(), win.right());
    Point2::new(x, y)
}

fn model(app: &App) -> Model {
    app.new_window().size(720, 720).build().unwrap();
    let n = 10;
    let mut points: VecDeque<Point2> = VecDeque::new();
    for _ in 0..n {
        points.push_back(random_point(app));
    }
    Model {
        points,
        n: 30,
        f: 0,
    }
}

fn update(app: &App, model: &mut Model, _: Update) {
    if model.f % STEP == STEP - 1 {
        model.points.push_back(random_point(app));
        if model.points.len() > model.n {
            model.points.pop_front().unwrap();
        }
    }
    model.f += 1;
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    let d = 100.0;
    let r = 0.95;
    for (i, p) in model.points.iter().enumerate() {
        let diameter = d * r.powi(i as i32);
        let a = map_range(i, 0, model.n - 1, 0.0, 1.0);
        draw.ellipse()
            .x_y(p.x, p.y)
            .w_h(diameter, diameter)
            .color(hsva(0.5, 1.0, 1.0, a));
    }
    draw.to_frame(app, &frame).unwrap();
}
