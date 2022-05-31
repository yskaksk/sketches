use nannou::image::{open, ImageBuffer, Rgb};
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run()
}

struct Dot {
    loc: Point2,
    direction: f32
}

struct Model {
    points: Vec<Dot>,
    texture: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

fn model(app: &App) -> Model {
    let assets = app.assets_path().unwrap();
    let img = open(assets.join("Dali_square.png")).unwrap().into_rgb8();
    app.new_window()
        .view(view)
        .size(img.width(), img.height())
        .build()
        .unwrap();
    let win = app.window_rect();
    let npoints = 100;
    let points = Vec::from_iter((0..npoints).map(|i| {
        let x = win.left();
        let y = map_range(i, 0, npoints - 1, win.bottom(), win.top());
        let r = 0.0;
        Dot {
            loc: pt2(x, y),
            direction: r
        }
    }));
    Model {
        points,
        texture: img
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(WHITE);
    }
    for (_i, p) in model.points.iter().enumerate() {
        let c = rgba(0.0, 0.1, 0.1, 0.01);
        draw.ellipse().x_y(p.loc.x, p.loc.y).w_h(10.0, 10.0).color(c);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn update(app: &App, model: &mut Model, _: Update) {
    let win = app.window_rect();
    let rrange_max = f32::PI();
    let width = model.texture.width();
    let height = model.texture.height();
    for p in model.points.iter_mut() {
        let i = map_range(p.loc.x, win.left(), win.right(), 0, width-1);
        let j = map_range(p.loc.y, win.top(), win.bottom(), 0, height-1);
        let pixel = model.texture.get_pixel(i, j);
        let gray = (pixel[0] as f32 * 0.222 + pixel[1] as f32 * 0.707 + pixel[2] as f32 * 0.071) / 256.0;
        let rrange = (gray + 0.01) * rrange_max;
        p.direction += random_range(-rrange, rrange);
        if p.direction < -rrange_max {
            p.direction += 2.0 * rrange_max;
        }
        if p.direction > rrange_max {
            p.direction -= 2.0 * rrange_max;
        }
        p.loc.x += 2.0 * p.direction.cos();
        p.loc.y += 2.0 * p.direction.sin();
        if p.loc.x > win.right() {
            p.loc.x -= win.w();
        }
        if p.loc.x < win.left() {
            p.loc.x += win.w()
        }
        if p.loc.y > win.top() {
            p.loc.y -= win.h();
        }
        if p.loc.y < win.bottom() {
            p.loc.y += win.h()
        }
    }
}
