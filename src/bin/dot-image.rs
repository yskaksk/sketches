use nannou::image::{open, ImageBuffer, Rgb};
use nannou::prelude::*;

fn main() {
    nannou::app(model).run()
}

struct Model {
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
    Model { texture: img }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let w = model.texture.width();
    let h = model.texture.height();

    let wcount = (win.w() / 10.0).ceil();
    let hcount = (win.h() / 10.0).ceil();
    let wstep = (w as f32 / wcount).floor() as usize;
    let hstep = (h as f32 / hcount).floor() as usize;

    if frame.nth() == 0 {
        frame.clear(WHITE);
        for i in (0..w).step_by(wstep) {
            for j in (0..h).step_by(hstep) {
                let p = model.texture.get_pixel(i, j);
                let gray =
                    (p[0] as f32 * 0.222 + p[1] as f32 * 0.707 + p[2] as f32 * 0.071) / 256.0;
                let x = map_range(i, 0, w, win.left(), win.right());
                let y = map_range(j, h, 0, win.bottom(), win.top());
                let r = 100.0;
                draw.ellipse()
                    .x_y(x, y)
                    .w_h(r * (1.0 - gray), r * (1.0 - gray))
                    .color(rgba(p[0], p[1], p[2], 40));
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
