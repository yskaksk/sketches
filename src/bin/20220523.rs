use itertools::Itertools;
use nannou::prelude::*;

fn main() {
    nannou::app(model).view(view).update(update).run()
}

struct Model {
    points: Vec<Point2>,
    circle: Vec<Point2>,
}

fn model(app: &App) -> Model {
    app.new_window().size(720, 720).build().unwrap();
    let win = app.window_rect();
    let points = (0..3000).map(|_| {
        let x = random_range(win.left(), win.right());
        let y = random_range(win.bottom(), win.top());
        pt2(x, y)
    });
    let w = win.w();
    let delta = w / 64.0;
    let circle = (0..4).rev().cartesian_product(0..4).map(|(i, j)| {
        let x = (w / 4.0) * j as f32 + w / 8.0 - w / 2.0 + random_range(-delta, delta);
        let y = (w / 4.0) * i as f32 + w / 8.0 - w / 2.0 + random_range(-delta, delta);
        pt2(x, y)
    });
    Model {
        points: Vec::from_iter(points),
        circle: Vec::from_iter(circle),
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let c = rgba(0.0, 0.1, 0.2, 0.8);
    for p in model.points.iter() {
        draw.ellipse().x_y(p.x, p.y).w_h(5.0, 5.0).color(c);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn update(app: &App, model: &mut Model, _: Update) {
    let win = app.window_rect();
    model.points.iter_mut().for_each(|p| {
        let r = random_range::<f32>(0.0, 1.0) * 2.0 * f32::PI();
        let mut closest = win.w() + win.h();
        for c in model.circle.iter() {
            let d = (c.x - p.x).abs() + (c.y - p.y).abs();
            if d < closest {
                closest = d;
            }
        }
        if closest < 80.0 {
            p.x += 3.0 * r.cos();
            p.y += 3.0 * r.sin();
        } else {
            p.x += 10.0 * r.cos();
            p.y += 10.0 * r.sin();
        }
        if p.x > win.right() {
            p.x -= win.w();
        }
        if p.x < win.left() {
            p.x += win.w();
        }
        if p.y > win.top() {
            p.y -= win.h();
        }
        if p.y < win.bottom() {
            p.y += win.h();
        }
    });
}
