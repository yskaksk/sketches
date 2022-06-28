use nannou::image::{open, ImageBuffer, Rgb};
use nannou::prelude::*;

use sketches::util::gif_save_path;

fn main() {
    nannou::app(model).run();
}

struct Model {
    texture: ImageBuffer<Rgb<u8>, Vec<u8>>,
}

fn model(app: &App) -> Model {
    let assets = app.assets_path().unwrap();
    let img = open(assets.join("jidori4_sq.png")).unwrap().into_rgb8();
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
    let w = model.texture.width() as f32;
    let h = model.texture.height() as f32;
    if frame.nth() % 10 == 0 {
        draw.background().color(BEIGE);
        let r = (frame.nth() as f32 / 50.0).sin() * 0.15 + 0.3;
        draw_shape(
            &draw,
            win,
            &model.texture,
            pt2(win.left(), win.bottom()),
            w,
            h,
            r,
            20.0,
            0,
        );
    }
    draw.to_frame(app, &frame).unwrap();
    if frame.nth() < 300 {
        let path = gif_save_path(app, frame.nth() as usize);
        app.main_window().capture_frame(&path);
        println!("file: {} saved!", path);
    }
}

fn draw_shape(
    draw: &Draw,
    win: Rect,
    texture: &ImageBuffer<Rgb<u8>, Vec<u8>>,
    lt: Point2,
    w: f32,
    h: f32,
    ratio: f32,
    min_w: f32,
    f: usize,
) {
    let mut count = f % 2;
    let mut pos = lt.clone();

    let mut width = w;
    let mut height = h;

    //let ratio = random_range(ratio - 0.1, ratio + 0.1).min(0.99).max(0.01);

    loop {
        if width < min_w {
            break;
        }
        if height < min_w {
            break;
        }
        if count > 10000 {
            break;
        }
        if count % 2 == 0 {
            loop {
                if pos.x + width * ratio < lt.x + w + 0.1 {
                    let x = pos.x + width * ratio * 0.5;
                    let y = pos.y + height * 0.5;

                    let c = choose_color(win, texture, x, y);
                    draw.rect().x_y(x, y).w_h(width * ratio, height).color(c);
                    draw_shape(
                        &draw,
                        win,
                        texture,
                        pos,
                        width * ratio,
                        height,
                        ratio,
                        min_w,
                        f + 1,
                    );
                    pos.x += width * ratio;
                } else {
                    break;
                }
            }
            width = lt.x + w - pos.x;
        }
        if count % 2 == 1 {
            loop {
                if pos.y + height * ratio < lt.y + h + 0.1 {
                    let x = pos.x + width * 0.5;
                    let y = pos.y + height * ratio * 0.5;
                    let c = choose_color(win, texture, x, y);
                    draw.rect().x_y(x, y).w_h(width, height * ratio).color(c);
                    draw_shape(
                        &draw,
                        win,
                        texture,
                        pos,
                        width,
                        height * ratio,
                        ratio,
                        min_w,
                        f + 1,
                    );
                    pos.y += height * ratio;
                } else {
                    break;
                }
            }
            height = lt.y + h - pos.y;
        }
        count += 1;
    }
}

fn choose_color(win: Rect, texture: &ImageBuffer<Rgb<u8>, Vec<u8>>, x: f32, y: f32) -> Rgb8 {
    let w = texture.width();
    let h = texture.height();

    let xi = map_range::<f32, u32>(x, win.left(), win.right(), 0, w);
    let yi = map_range::<f32, u32>(y, win.top(), win.bottom(), 0, h);

    let p = texture.get_pixel(xi, yi);
    return rgb8(p[0], p[1], p[2]);
}
