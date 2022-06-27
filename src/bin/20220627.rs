use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::prelude::*;

use nannou::rand::Rng;

fn main() {
    nannou::sketch(view).size(1000, 1000).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    let win = app.window_rect();
    let w = win.w();
    let h = win.h();

    let pos = pt2(win.left(), win.bottom());
    if frame.nth() == 0 {
        draw.background().color(CORNSILK);
        draw_shape(&draw, pos, w, h, 0.59, 5.0);
        draw_square(&draw, pt2(0.0, 0.0), w, rgb8(110, 110, 110));
    }
    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::S) {
        let path = format!("output/{}/image.png", app.exe_name().unwrap());
        app.main_window().capture_frame(&path);
        println!("file: {} saved!", path);
    }
}

fn draw_shape(draw: &Draw, lt: Point2, w: f32, h: f32, ratio: f32, min_w: f32) {
    let mut count = 0;
    let mut pos = lt;

    let mut width = w;

    let ratio = random_range(ratio - 0.1, ratio + 0.1).max(0.01).min(0.99);

    loop {
        if width < min_w {
            break;
        }
        if count % 2 == 0 {
            loop {
                if pos.x + width * ratio < lt.x + w + 0.1 {
                    draw.polygon()
                        .points([
                            pos,
                            pt2(pos.x + width * ratio, pos.y),
                            pt2(pos.x + width * ratio, pos.y + width),
                            pt2(pos.x, pos.y + width),
                        ])
                        .color(choose_color());
                    draw_shape(&draw, pos, width * ratio, width, ratio, min_w);
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
                    draw.polygon()
                        .points([
                            pos,
                            pt2(pos.x + width, pos.y),
                            pt2(pos.x + width, pos.y + width / ratio),
                            pt2(pos.x, pos.y + width / ratio),
                        ])
                        .color(choose_color());
                    draw_shape(&draw, pos, width, width / ratio, ratio, min_w);
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

fn draw_square(draw: &Draw, loc: Point2, size: f32, color: Rgb8) {
    let r = 0.95 * size;
    let mut points = Vec::from_iter((0..=180).map(|i| {
        let theta = i as f32 * (f32::PI() / 180.0);
        let x = 0.5 * r * theta.cos();
        let y = 0.5 * r * theta.sin();
        pt2(x, y)
    }));
    points.push(pt2(-size / 2.0, 0.0));
    points.push(pt2(-size / 2.0, size / 2.0));
    points.push(pt2(size / 2.0, size / 2.0));
    points.push(pt2(size / 2.0, 0.0));
    draw.x_y(loc.x, loc.y).polygon().points(points).color(color);

    let mut points = Vec::from_iter((0..=180).map(|i| {
        let theta = i as f32 * (f32::PI() / 180.0) + f32::PI();
        let x = 0.5 * r * theta.cos();
        let y = 0.5 * r * theta.sin();
        pt2(x, y)
    }));
    points.push(pt2(size / 2.0, 0.0));
    points.push(pt2(size / 2.0, -size / 2.0));
    points.push(pt2(-size / 2.0, -size / 2.0));
    points.push(pt2(-size / 2.0, 0.0));
    draw.x_y(loc.x, loc.y).polygon().points(points).color(color);
}

fn choose_color() -> Rgb<Srgb, u8> {
    let colors = [
        ALICEBLUE,
        BEIGE,
        BURLYWOOD,
        CADETBLUE,
        DARKOLIVEGREEN,
        DODGERBLUE,
        LIGHTGRAY,
        LIGHTSALMON,
        OLIVEDRAB,
    ];
    colors[nannou::rand::thread_rng().gen_range(0..colors.len())]
}
