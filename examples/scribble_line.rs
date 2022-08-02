use nannou::prelude::*;
use sketches::util::scribble_line;

fn main() {
    nannou::sketch(view).size(1000, 1000).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(BEIGE);
        scribble_line(&draw, pt2(250.0, 250.0), pt2(250.0, -250.0));
        scribble_line(&draw, pt2(250.0, -250.0), pt2(-250.0, -250.0));
        scribble_line(&draw, pt2(-250.0, -250.0), pt2(-250.0, 250.0));
        scribble_line(&draw, pt2(-250.0, 250.0), pt2(250.0, 250.0));
    }
    draw.to_frame(app, &frame).unwrap();
}
