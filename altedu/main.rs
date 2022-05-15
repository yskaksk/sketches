use nannou::prelude::*;
use std::f32;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    current: Point2,
    old: Point2,
}

fn update(_: &App, model: &mut Model, _: Update) {
    let tmp = model.current.clone();
    model.current = choose(&model.current);
    model.old = tmp;
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(800, 800)
        .title("circle")
        .view(view)
        .build()
        .unwrap();
    Model {
        current: Point2::new(250.0, 0.0),
        old: Point2::new(250.0, 0.0),
    }
}

fn choose(current: &Point2) -> Point2 {
    let mut rad = if current.x >= 0.0 {
        (current.y / (current.x + 0.01)).atan()
    } else {
        f32::consts::PI - (current.y / -current.x).atan()
    };
    rad -= f32::consts::PI / 2.0 + 0.015;
    let r = random_f32();
    let (theta, d) = if r > 0.4 {
        (rad, 10.0)
    } else if r > 0.3 {
        (rad + f32::consts::PI, 5.0)
    } else if r > 0.15 {
        (rad + f32::consts::PI / 2.0, 3.0)
    } else {
        (rad - f32::consts::PI / 2.0, 3.0)
    };
    return Point2::new(current.x + d * theta.cos(), current.y + d * theta.sin());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(WHITE);
    }
    draw.line().start(model.old).end(model.current);
    draw.to_frame(app, &frame).unwrap();
}
