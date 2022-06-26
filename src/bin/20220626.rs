use nannou::image::{open, ImageBuffer, Rgb};
use nannou::prelude::*;

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
    draw.background().color(WHITE);
    let win = app.window_rect();
    let w = model.texture.width();
    let h = model.texture.height();

    for i in (0..w).step_by(10) {
        for j in (0..h).step_by(10) {
            let x = map_range(i, 0, w - 1, win.left(), win.right());
            let y = map_range(j, 0, h - 1, win.top(), win.bottom());
            let px = model.texture.get_pixel(i, j);
            let gray = (px[0] as f32 * 0.222 + px[1] as f32 * 0.707 + px[2] as f32 * 0.071) / 256.0;
            draw_shape(
                &draw,
                20.0,
                pt2(x, y),
                1.0 - gray,
                rgb8(px[0], px[1], px[2]),
            );
        }
    }
    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::S) {
        let path = format!("output/{}/image.png", app.exe_name().unwrap());
        app.main_window().capture_frame(&path);
        println!("file: {} saved!", path);
    }
}

fn draw_shape(draw: &Draw, size: f32, loc: Point2, gray: f32, color: Rgb8) {
    let d = size / 4.0;
    let sq2 = 2.0.sqrt();
    let theta = f32::PI() / 4.0;
    let n = (gray / (1.0 / 16.0)).floor() as usize;
    let mut c = 0;
    for i in 0..3 {
        if c >= n {
            break;
        }
        for j in 0..3 {
            let x = loc.x - size * 0.5 + d * i as f32;
            let y = loc.y - size * 0.5 + d * j as f32;
            draw.x_y(x, y)
                .rect()
                .w_h(d / sq2, d / sq2)
                .rotate(theta)
                .color(color);
            c += 1;
        }
    }
}
