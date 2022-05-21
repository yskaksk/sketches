use nannou::prelude::*;

fn main() {
    nannou::app(model).view(view).update(update).run()
}

#[derive(Clone)]
struct Line {
    points: Vec<Point2>,
}

impl Line {
    fn new() -> Self {
        Line { points: vec![] }
    }
    fn push(&mut self, p: Point2) {
        self.points.push(p)
    }
    fn len(&self) -> usize {
        self.points.len()
    }
}

struct Model {
    lines: Vec<Line>,
    line: Line,
    noise: bool,
    drawing: bool,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(720, 720)
        .mouse_pressed(mouse_pressed)
        .mouse_moved(mouse_moved)
        .mouse_released(mouse_released)
        .key_pressed(key_pressed)
        .build()
        .unwrap();
    Model {
        lines: vec![],
        line: Line::new(),
        noise: false,
        drawing: false,
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(WHITE);
    }

    let c = rgba(0.3, 0.3, 0.3, 0.01);
    for l in model.lines.iter() {
        draw.polyline()
            .weight(1.0)
            .points(l.points.clone())
            .color(c);
    }
    if model.line.len() > 0 {
        draw.polyline()
            .weight(3.0)
            .points(model.line.points.clone())
            .color(c);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn update(_: &App, model: &mut Model, _: Update) {
    if model.noise {
        let lines = model.lines.iter().map(|l| {
            let points = l.points.iter().map(|p| {
                let x = p.x + random_range(-0.5, 0.5);
                let y = p.y + random_range(-0.5, 0.5);
                pt2(x, y)
            });
            Line {
                points: Vec::from_iter(points),
            }
        });
        model.lines = Vec::from_iter(lines)
    }
}

fn mouse_pressed(_: &App, model: &mut Model, _: MouseButton) {
    model.drawing = true;
}

fn mouse_moved(_: &App, model: &mut Model, pos: Point2) {
    if model.drawing {
        model.line.push(pos);
    }
}

fn mouse_released(_: &App, model: &mut Model, _: MouseButton) {
    model.drawing = false;
    model.lines.push(model.line.clone());
    model.line = Line::new();
}

fn key_pressed(_: &App, model: &mut Model, key: Key) {
    match key {
        Key::N => {
            model.noise = !model.noise;
        }
        _ => {}
    }
}
