use nannou::prelude::*;
use nannou::rand;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    x: f32,
    y: f32,
}

fn model(app: &App) -> Model {
    app.new_window().view(view).size(600, 600).build().unwrap();
    Model { x: 0.0, y: 0.0 }
}

fn update(app: &App, model: &mut Model, _: Update) {
    let win = app.window_rect();
    let r = rand::random_f32();
    if r < 0.25 {
        model.x += 2.0;
    } else if r < 0.5 {
        model.y += 2.0;
    } else if r < 0.75 {
        model.x -= 2.0;
    } else {
        model.y -= 2.0;
    }
    if model.x > win.right() {
        model.x -= win.w();
    }
    if model.x < win.left() {
        model.x += win.w();
    }
    if model.y > win.top() {
        model.y -= win.h();
    }
    if model.y < win.bottom() {
        model.y += win.h();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(BLACK);
    }
    let t = frame.nth() % 360;
    let c = hsv(map_range(t, 0, 359, 0.0, 1.0), 1.0, 1.0);
    draw.rect().x_y(model.x, model.y).w_h(2.0, 2.0).color(c);

    draw.to_frame(app, &frame).unwrap();
}
