use nannou::prelude::*;
use sketches::util::{cyclical_rect, random_point};

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    hours: Vec<Point2>,
    minutes: Vec<Point2>,
    seconds: Vec<Point2>,
    hour: usize,
    minute: usize,
    second: usize,
    fc: usize,
}

fn model(app: &App) -> Model {
    app.new_window().size(1500, 500).view(view).build().unwrap();
    let win = app.window_rect();
    let hwin = Rect::from_x_y_w_h(-win.w() / 3.0, 0.0, win.w() / 3.0, win.h());
    let hours = Vec::from_iter((0..24).map(|_| random_point(hwin)));
    let mwin = Rect::from_x_y_w_h(0.0, 0.0, win.w() / 3.0, win.h());
    let minutes = Vec::from_iter((0..60).map(|_| random_point(mwin)));
    let hwin = Rect::from_x_y_w_h(win.w() / 3.0, 0.0, win.w() / 3.0, win.h());
    let seconds = Vec::from_iter((0..60).map(|_| random_point(hwin)));

    Model {
        hours,
        minutes,
        seconds,
        hour: 2,
        minute: 30,
        second: 0,
        fc: 0,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    for s in model.seconds[0..model.second].iter() {
        draw.ellipse()
            .w_h(30.0, 30.0)
            .color(rgb(0.2, 0.2, 0.9))
            .x_y(s.x, s.y);
    }
    for m in model.minutes[0..model.minute].iter() {
        draw.ellipse()
            .w_h(50.0, 50.0)
            .color(rgb(0.1, 0.8, 0.3))
            .x_y(m.x, m.y);
    }
    for h in model.hours[0..model.hour].iter() {
        draw.ellipse()
            .w_h(100.0, 100.0)
            .color(rgb(0.9, 0.2, 0.2))
            .x_y(h.x, h.y);
    }
    if app.keys.down.contains(&Key::S) {
        let path = format!("output/{}/image.png", app.exe_name().unwrap());
        app.main_window().capture_frame(&path);
        println!("file: {} saved!", path);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn update(app: &App, model: &mut Model, _: Update) {
    if model.fc % 5 == 0 {
        model.second += 1;
        if model.second == 60 {
            model.minute += 1;
            model.second = 0;
        }
        if model.minute == 60 {
            model.hour += 1;
            model.minute = 0;
        }
        if model.hour == 24 {
            model.hour = 0;
        }
    }
    let win = app.window_rect();
    let hwin = Rect::from_x_y_w_h(-win.w() / 3.0, 0.0, win.w() / 3.0, win.h());
    for p in model.hours.iter_mut() {
        p.x += random_range(-1.0, 1.0) * 5.0;
        p.y += random_range(-1.0, 1.0) * 5.0;
        *p = cyclical_rect(p.clone(), hwin);
    }
    let mwin = Rect::from_x_y_w_h(0.0, 0.0, win.w() / 3.0, win.h());
    for p in model.minutes.iter_mut() {
        p.x += random_range(-1.0, 1.0) * 10.0;
        p.y += random_range(-1.0, 1.0) * 10.0;
        *p = cyclical_rect(p.clone(), mwin);
    }
    let hwin = Rect::from_x_y_w_h(win.w() / 3.0, 0.0, win.w() / 3.0, win.h());
    for p in model.seconds.iter_mut() {
        let pc = p.clone();
        p.x += pc.y * 0.01 + random_range(-1.0, 1.0) * 50.0;
        p.y += pc.x * 0.01 + random_range(-1.0, 1.0) * 50.0;
        *p = cyclical_rect(p.clone(), hwin);
    }
    model.fc += 1;
}
