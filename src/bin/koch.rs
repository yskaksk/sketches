use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(1000, 1000).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(BEIGE);
        let mut points = vec![pt2(0.0, 0.0), pt2(1000.0, 0.0)];
        for _ in 0..10 {
            points = gen(1000.0 / 3.0, points);
        }
        draw.x_y(-500.0, -50.0).polyline().points(points);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn gen(d: f32, points: Vec<Point2>) -> Vec<Point2> {
    let theta = f32::PI() / 3.0;
    let mut one = Vec::from_iter(points.iter().map(|p| {
        let x = p.x / 3.0;
        let y = p.y / 3.0;
        pt2(x, y)
    }));
    let two = Vec::from_iter(points.iter().map(|p| {
        let x = p.x / 3.0;
        let y = p.y / 3.0;
        pt2(
            x * theta.cos() - y * theta.sin() + d,
            x * theta.sin() + y * theta.cos(),
        )
    }));
    let three = Vec::from_iter(points.iter().map(|p| {
        let x = p.x / 3.0;
        let y = p.y / 3.0;
        pt2(
            x * (-theta).cos() - y * (-theta).sin() + 1.5 * d,
            x * (-theta).sin() + y * (-theta).cos() + d * 0.5 * 3.0.sqrt(),
        )
    }));
    let four = Vec::from_iter(points.iter().map(|p| {
        let x = p.x / 3.0;
        let y = p.y / 3.0;
        pt2(x + 2.0 * d, y)
    }));
    one.extend(&two);
    one.extend(&three);
    one.extend(&four);
    return one;
}
