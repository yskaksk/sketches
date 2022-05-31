use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run()
}

struct Dot {
    loc: Point2,
    direction: f32,
}

struct Model {
    points: Vec<Dot>,
}

fn model(app: &App) -> Model {
    app.new_window().view(view).size(720, 720).build().unwrap();
    let win = app.window_rect();
    let npoints = 100;
    let points = Vec::from_iter((0..npoints).map(|i| {
        let x = win.left();
        let y = map_range(i, 0, npoints - 1, win.bottom(), win.top());
        let r = 0.0;
        Dot {
            loc: pt2(x, y),
            direction: r,
        }
    }));
    Model { points }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(WHITE);
    }
    for (_i, p) in model.points.iter().enumerate() {
        let c = rgba(0.0, 0.1, 0.1, 0.4);
        draw.ellipse().x_y(p.loc.x, p.loc.y).w_h(3.0, 3.0).color(c);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn update(app: &App, model: &mut Model, _: Update) {
    let win = app.window_rect();
    let rrange = 0.4 * f32::PI();
    for p in model.points.iter_mut() {
        if (p.loc.x + 180.0).abs() > 60.0 && (p.loc.x - 180.0).abs() > 60.0 {
            p.direction += random_range(-rrange, rrange);
        } else {
            p.direction = 0.0;
        }
        if p.direction < -rrange {
            p.direction += 2.0 * rrange;
        }
        if p.direction > rrange {
            p.direction -= 2.0 * rrange;
        }
        p.loc.x += 2.0 * p.direction.cos();
        p.loc.y += 2.0 * p.direction.sin();
        if p.loc.y > win.top() {
            p.loc.y -= win.h();
        }
        if p.loc.y < win.bottom() {
            p.loc.y += win.h()
        }
    }
}
