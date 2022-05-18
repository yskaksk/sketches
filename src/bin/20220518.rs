use nannou::prelude::*;
use std::collections::VecDeque;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    points: VecDeque<Point2>,
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
    let win = app.window_rect();
    Model {
        points: VecDeque::from([Point2::new(win.h() / 3.0, 0.0)]),
        d: 100.0,
        n: 40,
        f: 0,
        savecount: 0,
        saving: false,
    }
}

fn update(app: &App, model: &mut Model, _: Update) {
    if model.f % 5 == 0 {
        let win = app.window_rect();
        let x = (win.h() / 3.0) * (7.0 * model.f as f32 * f32::PI() / 360.0).cos()
            + 50.0 * (model.f as f32 / 12.0).cos();
        let y = (win.h() / 3.0) * (3.0 * model.f as f32 * f32::PI() / 360.0).sin()
            + 50.0 * (model.f as f32 / 12.0).sin();
        model.points.push_back(Point2::new(x, y));
        if model.points.len() > model.n {
            model.points.pop_front().unwrap();
        }
    }
    model.f += 1;
    if model.f == 360 {
        model.saving = true;
        println!("start to save");
    }
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
    if model.savecount == 720 {
        model.saving = false;
        println!("saved!");
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let r: f32 = 0.98;
    for (i, p) in model.points.iter().enumerate() {
        let diameter = model.d * r.powi(model.points.len() as i32 - i as i32);
        let h = map_range(i, 0, model.points.len() - 1, 0.1, 1.0);
        let a = map_range(i, 0, model.points.len() - 1, 0.01, 1.0);
        draw.ellipse()
            .x_y(p.x, p.y)
            .w_h(diameter, diameter)
            .color(hsva(h, 1.0, 1.0, a));
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
