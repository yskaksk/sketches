use nannou::prelude::*;
use sketches::util::curve_vertex;

fn main() {
    nannou::sketch(view).size(1000, 1000).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BEIGE);

    let p1 = pt2(170.0, 205.0);
    let p2 = pt2(90.0, -155.0);
    let p3 = pt2(-145.0, -165.0);
    let p4 = pt2(-90.0, 205.0);

    draw.ellipse().xy(p1).w_h(10.0, 10.0).color(BLACK);
    draw.ellipse().xy(p2).w_h(10.0, 10.0).color(BLACK);
    draw.ellipse().xy(p3).w_h(10.0, 10.0).color(BLACK);
    draw.ellipse().xy(p4).w_h(10.0, 10.0).color(BLACK);

    curve_vertex(&draw, p1, p1, p2, p3);
    curve_vertex(&draw, p1, p2, p3, p4);
    curve_vertex(&draw, p2, p3, p4, p4);

    draw.to_frame(app, &frame).unwrap();
}
