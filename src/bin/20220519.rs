use nannou::prelude::*;

fn main() {
    nannou::app(model).view(view).update(update).run()
}

struct Model {
    points: Vec<Point2>
}

const N : usize = 60;

fn model(app: &App) -> Model {
    app.new_window()
        .size(720, 720)
        .build()
        .unwrap();
    let points = (0..=N).map(|i| {
        let theta = i as f32 * 2.0 * f32::PI() / N as f32;
        let x = 200.0 * theta.cos();
        let y = 200.0 * theta.sin();
        pt2(x, y)
    });
    Model {
        points: Vec::from_iter(points)
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(WHITE);
    }
    if frame.nth() < 600 {
        draw.polyline().weight(3.0).points(model.points.clone()).color(rgba(0.5, 0.5, 0.5, 0.01));
        draw.to_frame(app, &frame).unwrap();
    }
}

fn update(_: &App, model: &mut Model, _: Update) {
    let points = model.points.iter().map(|p| {
        let x = p.x + random_range(-1.0, 1.0);
        let y = p.y + random_range(-1.0, 1.0);
        pt2(x, y)
    });
    model.points = Vec::from_iter(points);
}
