use nannou::prelude::*;
use nannou_touchosc::TouchOscClient;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    touchosc: TouchOscClient,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1000, 1000)
        .view(view)
        .build()
        .unwrap();

    let mut touchosc = TouchOscClient::new(6555);

    touchosc.add_fader("/color_r", 0.0, 1.0, 0.5);
    touchosc.add_fader("/color_g", 0.0, 1.0, 0.5);
    touchosc.add_fader("/color_b", 0.0, 1.0, 0.5);
    touchosc.add_fader("/color_a", 0.0, 1.0, 0.5);
    touchosc.add_fader("/stroke_width", 1.0, 3.0, 1.0);
    touchosc.add_fader("/vertices", 0.0, 1.0, 1.0);

    touchosc.add_xy("/scale", -1.0, 1.0, 0.0);
    touchosc.add_encoder("/rotate", 0.0, 2.0 * f32::PI(), 0.0);

    Model { touchosc }
}

fn update(_: &App, model: &mut Model, _: Update) {
    model.touchosc.update();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let b = model.touchosc.fader("/vertices");
    draw.background().color(rgb(b, b, b));
    let c = rgba(
        model.touchosc.fader("/color_r"),
        model.touchosc.fader("/color_g"),
        model.touchosc.fader("/color_b"),
        model.touchosc.fader("/color_a"),
    );
    let gap = model.touchosc.xy("/scale");
    let rot = model.touchosc.encoder("/rotate");
    let r = model.touchosc.fader("/stroke_width");

    let n = 5;
    let size = win.w() / n as f32;
    for i in 0..n {
        for j in 0..n {
            let x = size * 0.5 + (i as f32) * size - win.w() * 0.5;
            let y = size * 0.5 + (j as f32) * size - win.h() * 0.5;
            draw_shape(&draw, pt2(x, y), rot, r * size, gap, c);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn draw_shape(draw: &Draw, loc: Point2, rot: f32, size: f32, gap: Point2, color: Rgba<f32>) {
    let r = 0.8 * size;
    let mut points = Vec::from_iter((0..=180).map(|i| {
        let theta = i as f32 * (f32::PI() / 180.0);
        let x = 0.5 * r * theta.cos() + size * 0.1 * gap.x;
        let y = 0.5 * r * theta.sin() + size * 0.1 * gap.y;
        pt2(x, y)
    }));
    points.push(pt2(-size / 2.0, 0.0));
    points.push(pt2(-size / 2.0, size / 2.0));
    points.push(pt2(size / 2.0, size / 2.0));
    points.push(pt2(size / 2.0, 0.0));
    draw.x_y(loc.x, loc.y)
        .rotate(rot)
        .polygon()
        .points(points)
        .color(color);

    let mut points = Vec::from_iter((0..=180).map(|i| {
        let theta = i as f32 * (f32::PI() / 180.0) + f32::PI();
        let x = 0.5 * r * theta.cos() + size * 0.1 * gap.x;
        let y = 0.5 * r * theta.sin() + size * 0.1 * gap.y;
        pt2(x, y)
    }));
    points.push(pt2(size / 2.0, 0.0));
    points.push(pt2(size / 2.0, -size / 2.0));
    points.push(pt2(-size / 2.0, -size / 2.0));
    points.push(pt2(-size / 2.0, 0.0));
    draw.x_y(loc.x, loc.y)
        .rotate(rot)
        .polygon()
        .points(points)
        .color(color);
}
