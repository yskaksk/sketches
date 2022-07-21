use nannou::prelude::*;
use sketches::util::{curve, random_from_circle};

fn main() {
    nannou::sketch(view).size(1000, 1000).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(BEIGE);
        for i in 0..5 {
            let r = map_range(i, 0, 4, 200.0, 500.0);
            let points = Vec::from_iter((0..100).map(|_| random_from_circle() * r));
            curve(&draw, points);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
