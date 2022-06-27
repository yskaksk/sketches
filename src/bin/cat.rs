use nannou::prelude::*;

fn main() {
    nannou::sketch(view).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.to_frame(app, &frame).unwrap();
}

fn cat(draw: &Draw, loc: Point2, size: f32) {}
