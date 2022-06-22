use nannou::image::{open, ImageBuffer, Rgb};
use nannou::prelude::*;

use sketches::util::random_point;

fn main() {
    nannou::app(model).run();
}

struct Model {
    texture: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

fn model(app: &App) -> Model {
    let assets = app.assets_path().unwrap();
    let img = open(assets.join("jidori3.png")).unwrap().into_rgb8();
    app.new_window()
        .view(view)
        .size(img.width(), img.height())
        .build()
        .unwrap();
    Model { texture: img }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let w = model.texture.width();
    let h = model.texture.height();

    let d = 15.0;
    let wcount = (win.w() / d).ceil();
    let hcount = (win.h() / d).ceil();
    let wstep = (w as f32 / wcount).floor() as usize;
    let hstep = (h as f32 / hcount).floor() as usize;
    if frame.nth() == 0 {
        draw.background().color(WHITE);
        for i in (0..w).step_by(wstep) {
            for j in (0..h).step_by(hstep) {
                let p = model.texture.get_pixel(i, j);
                let g = (p[0] as f32 * 0.222 + p[1] as f32 * 0.707 + p[2] as f32 * 0.071) / 256.0;
                let x = map_range(i, 0, w - 1, win.left(), win.right());
                let y = map_range(j, 0, h - 1, win.top(), win.bottom());
                //draw_shape1(&draw, d * 1.05, pt2(x, y), g);
                draw_shape2(&draw, d * 1.5, pt2(x, y), g, rgb8(p[0], p[1], p[2]));
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::S) {
        let path = format!("output/{}/image.png", app.exe_name().unwrap());
        app.main_window().capture_frame(&path);
        println!("file: {} saved!", path);
    }
}

fn draw_shape1(draw: &Draw, size: f32, loc: Point2, gray: f32) {
    let n = 30;
    for i in 0..n {
        let r = map_range(i, 0, n - 1, size, size / (n as f32)) * 0.5;
        let b = map_range(i, 0, n - 1, gray * 0.5, gray);
        draw.polygon()
            .points(vec![pt2(r, r), pt2(-r, r), pt2(-r, -r), pt2(r, -r)])
            .x_y(loc.x, loc.y)
            .color(rgb(b, b, b));
    }
}

fn draw_shape2(draw: &Draw, size: f32, loc: Point2, gray: f32, color: Rgb8) {
    let n = (30.0 * (1.0 - gray)).ceil() as usize;
    let r = size / 3.0;
    let rct = Rect::from_x_y_w_h(loc.x, loc.y, r, r);
    for _ in 0..n {
        let p = random_point(rct);
        draw.ellipse().x_y(p.x, p.y).w_h(r, r).color(color);
    }
}
