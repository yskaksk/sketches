use nannou::image::{open, ImageBuffer, Rgb};
use nannou::prelude::*;

mod text_wg;
use text_wg::WG;

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

    let mut wc = 0;
    let len = WG.len();

    if frame.nth() == 0 {
        draw.background().color(WHITE);
        for i in (0..w).step_by(10) {
            for j in (0..h).step_by(10) {
                let p = model.texture.get_pixel(i, j);
                let g = (p[0] as f32 * 0.222 + p[1] as f32 * 0.707 + p[2] as f32 * 0.071) / 256.0;
                let x = map_range(i, 0, w - 1, win.left(), win.right());
                let y = map_range(j, 0, h - 1, win.top(), win.bottom());
                let l = WG.chars().nth(wc % len).unwrap();
                draw.text(&l.to_string())
                    .color(rgb8(p[0], p[1], p[2]))
                    .font_size(100)
                    .x_y(x, y)
                    .w_h(10.0, 10.0)
                    .align_text_top();
                wc += 1;
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
