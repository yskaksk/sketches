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
        let r = (app.time / 100.0).sin() * 0.45 + 0.5;
        //let r = 0.69;
        draw_shape(
            &draw,
            win,
            &model.texture,
            pt2(win.left(), win.bottom()),
            w,
            h,
            r,
            10.0,
        );
    }
    draw.to_frame(app, &frame).unwrap();
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
) {
    let mut count = 0;
    let mut pos = lt;

    let mut width = w;

    //let ratio = random_range(ratio - 0.1, ratio + 0.1).max(0.01).min(0.99);

    loop {
        if width < min_w {
            break;
        }
        if count % 2 == 0 {
            loop {
                if pos.x + width * ratio < lt.x + w + 0.1 {
                    let x = pos.x + width * ratio * 0.5;
                    let y = pos.y + width * 0.5;

                    let c = choose_color(win, texture, x, y);
                    draw.rect().x_y(x, y).w_h(width * ratio, width).color(c);
                    draw_shape(&draw, win, texture, pos, width * ratio, width, ratio, min_w);
                    pos.x += width * ratio;
                } else {
                    break;
                }
            }
            width = lt.x + w - pos.x;
        }
        if count % 2 == 1 {
            loop {
                if pos.y + width / ratio < lt.y + h + 0.1 {
                    let x = pos.x + width * 0.5;
                    let y = pos.y + width / ratio * 0.5;
                    let c = choose_color(win, texture, x, y);
                    draw.rect().x_y(x, y).w_h(width, width / ratio).color(c);
                    draw_shape(&draw, win, texture, pos, width, width / ratio, ratio, min_w);
                    pos.y += width / ratio;
                } else {
                    break;
                }
            }
            width = lt.y + h - pos.y;
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
