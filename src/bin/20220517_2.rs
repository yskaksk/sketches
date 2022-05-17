use nannou::prelude::*;
use std::collections::VecDeque;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    points: VecDeque<Point2>,
    theta: f32,
    d: f32,
    n: usize,
    f: usize,
    savecount: usize,
    saving: bool,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(720, 720)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    Model {
        points: VecDeque::from([Point2::new(0.0, 0.0)]),
        theta: 0.0,
        d: 100.0,
        n: 30,
        f: 0,
        savecount: 0,
        saving: false,
    }
}

fn update(app: &App, model: &mut Model, _: Update) {
    if model.f % 5 == 4 {
        let win = app.window_rect();
        let head = model.points.back().unwrap();
        let e = ((model.f as f32) / 10.0).powf(2.0).sin();
        let mut x = head.x + 0.5 * model.d * (model.theta + e).cos();
        let mut y = head.y + 0.5 * model.d * (model.theta + e).sin();
        if x > win.right() {
            x -= win.w();
        }
        if x < win.left() {
            x += win.w();
        }
        if y > win.top() {
            y -= win.h();
        }
        if y < win.bottom() {
            y += win.h();
        }
        model.points.push_back(Point2::new(x, y));
        if model.points.len() > model.n {
            model.points.pop_front().unwrap();
        }
        model.theta += e;
    }
    model.f += 1;
    if model.saving {
        let path = format!(
            "output/{}/image_{:>03}.png",
            app.exe_name().unwrap(),
            model.savecount
        );
        app.main_window().capture_frame(path);
        println!("saving {}", model.savecount);
        model.savecount += 1;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let r: f32 = 0.95;
    for (i, p) in model.points.iter().enumerate() {
        let diameter = model.d * r.powi(model.points.len() as i32 - i as i32);
        let h = map_range(i, 0, model.points.len() - 1, 0.0, 1.0);
        draw.ellipse()
            .x_y(p.x, p.y)
            .w_h(diameter, diameter)
            .color(hsv(h, 1.0, 1.0));
    }
    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(_: &App, model: &mut Model, key: Key) {
    match key {
        Key::S => {
            model.saving = !model.saving;
        }
        _ => {}
    }
}
