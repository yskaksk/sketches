use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run()
}

struct Model {
    hour: usize,
    minute: usize,
    second: usize,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(1000, 1000)
        .view(view)
        .build()
        .unwrap();
    Model {
        hour: 1,
        minute: 0,
        second: 0,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let win = app.window_rect();
    let s = win.w() / 12.0;

    let c_hour = rgba::<f32>(0.8, 0.3, 0.3, 1.0);
    let c_minute = rgba::<f32>(0.3, 0.9, 0.3, 1.0);
    let c_second = rgba::<f32>(0.1, 0.1, 0.9, 1.0);
    let c_none = rgba::<f32>(0.3, 0.3, 0.3, 0.3);

    for i in 0..12 {
        for j in 0..2 {
            let x = s * 0.5 + i as f32 * s - win.w() * 0.5;
            let y = win.top() - (s * 0.5 + j as f32 * s);
            let c = if (i + 12 * j) < model.hour {
                c_hour
            } else {
                c_none
            };
            draw.x_y(x, y).rect().w_h(s * 0.9, s * 0.9).color(c);
        }
        for j in 2..7 {
            let x = s * 0.5 + i as f32 * s - win.w() * 0.5;
            let y = win.top() - (s * 0.5 + j as f32 * s);
            let c = if (i + 12 * j) < (24 + model.minute) {
                c_minute
            } else {
                c_none
            };
            draw.x_y(x, y).rect().w_h(s * 0.9, s * 0.9).color(c);
        }
        for j in 7..12 {
            let x = s * 0.5 + i as f32 * s - win.w() * 0.5;
            let y = win.top() - (s * 0.5 + j as f32 * s);
            let c = if (i + 12 * j) < (84 + model.second) {
                c_second
            } else {
                c_none
            };
            draw.x_y(x, y).rect().w_h(s * 0.9, s * 0.9).color(c);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn update(_: &App, model: &mut Model, _: Update) {
    model.second += 1;
    if model.second == 60 {
        model.second = 0;
        model.minute += 1;
    }
    if model.minute == 60 {
        model.minute = 0;
        model.hour += 1;
    }
    if model.hour == 24 {
        model.hour = 0;
    }
}
