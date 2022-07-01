use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::prelude::*;
use nannou::rand::Rng;

use sketches::util::random_point;

fn main() {
    nannou::app(model).update(update).run();
}

struct Circle {
    center: Point2,
    radius: f32,
    color: Rgb<Srgb, u8>,
}

struct Model {
    circles: Vec<Circle>,
    full: bool,
    updated: bool,
}

fn model(app: &App) -> Model {
    app.new_window()
        .view(view)
        .size(1000, 1000)
        .build()
        .unwrap();
    Model {
        circles: vec![],
        full: false,
        updated: false,
    }
}

fn update(app: &App, model: &mut Model, _: Update) {
    model.updated = false;
    if model.circles.len() > 150 {
        model.full = true;
        return;
    }
    let win = app.window_rect();
    let mut count = 0;
    loop {
        count += 1;
        if count > 1000 {
            break;
        }
        let p = random_point(win);
        let mut minimum = win.h() + win.w();
        let mut addable = true;
        for c in model.circles.iter() {
            let d = c.center.distance(p) - c.radius;
            if d < minimum {
                minimum = d;
            }
            if d < 15.0 {
                addable = false;
                break;
            }
        }
        if addable {
            let r = minimum.min(200.0);
            let c = choose_color();
            model.circles.push(Circle {
                center: p,
                radius: r,
                color: c,
            });
            model.updated = true;
            break;
        } else {
            continue;
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(BEIGE);
    }
    if !model.full && model.updated {
        if let Some(c) = model.circles.last() {
            draw_shape(&draw, c.center, c.radius * 1.9, c.color);
        }
    }
    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::S) {
        let path = format!("output/{}/image.png", app.exe_name().unwrap());
        app.main_window().capture_frame(&path);
        println!("file: {} saved!", path);
    }
}

fn draw_shape(draw: &Draw, loc: Point2, size: f32, color: Rgb<Srgb, u8>) {
    let rad = size * 0.5;
    let pr = 4.0;
    let n = ((rad / (pr / 2.0)).powf(2.0) * 0.8).floor() as u32;
    for _ in 0..n.min(3000) {
        let z = -2.0 * random::<f32>() + 1.0;
        let v = random::<f32>();
        let r = (1.0 - z.powf(2.0)).sqrt();
        let x = r * (2.0 * f32::PI() * v).cos();
        let y = r * (2.0 * f32::PI() * v).sin();
        draw.x_y(loc.x, loc.y)
            .ellipse()
            .x_y(rad * x, rad * y)
            .w_h(pr, pr)
            .color(color);
    }
}

fn choose_color() -> Rgb<Srgb, u8> {
    let colors = [
        DARKRED,
        CADETBLUE,
        DARKBLUE,
        DARKOLIVEGREEN,
        DODGERBLUE,
        LIGHTSALMON,
        TOMATO,
    ];
    colors[nannou::rand::thread_rng().gen_range(0..colors.len())]
}
