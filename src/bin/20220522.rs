use itertools::Itertools;
use nannou::prelude::*;

fn main() {
    nannou::app(model).view(view).run()
}

struct Eye {
    loc: Point2,
    radius: f32,
    direction: f32,
}

impl Eye {
    fn draw(&self, draw: &Draw) {
        let inner_x = self.loc.x + self.radius * 0.5 * self.direction.cos();
        let inner_y = self.loc.y + self.radius * 0.5 * self.direction.sin();
        let cb = hsva(0.8, 1.0, 1.0, 0.5);
        let ci = hsva(0.3, 1.0, 1.0, 0.5);
        draw.ellipse()
            .x_y(self.loc.x, self.loc.y)
            .w_h(2.0 * self.radius, 2.0 * self.radius)
            .color(cb);
        draw.ellipse()
            .x_y(inner_x, inner_y)
            .w_h(self.radius, self.radius)
            .color(ci);
    }
}

struct Model {
    eyes: Vec<Eye>,
}

fn model(app: &App) -> Model {
    app.new_window().size(720, 720).build().unwrap();

    let win = app.window_rect();
    let radius = win.w() / 40.0;
    let eyes = Vec::from_iter((0..20).cartesian_product(0..20).map(|(i, j)| {
        let x = map_range(i, 0, 19, win.left(), win.right()) + 2.0 * radius * (random_f32() - 0.5);
        let y = map_range(j, 0, 19, win.bottom(), win.top()) + 2.0 * radius * (random_f32() - 0.5);
        let direction = random_f32() * 2.0 * f32::PI();
        Eye {
            loc: pt2(x, y),
            radius,
            direction,
        }
    }));

    Model { eyes }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for e in model.eyes.iter() {
        e.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
