use nannou::prelude::*;

use sketches::util::random_point;

fn main() {
    nannou::sketch(view).size(1000, 1000).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let win = app.window_rect();
    draw_shape(
        &draw,
        pt2(0.0, 0.0),
        600.0,
        rgb(0.8, 0.6, 0.8),
        rgb(0.9, 0.9, 0.7),
    );
    draw.to_frame(app, &frame).unwrap();
}

fn draw_shape(draw: &Draw, loc: Point2, r: f32, oc: Rgb<f32>, ic: Rgb<f32>) {
    draw.x_y(loc.x, loc.y).ellipse().w_h(r, r).color(oc);
    draw.x_y(loc.x, loc.y)
        .ellipse()
        .w_h(0.3 * r, 0.3 * r)
        .color(ic);
    draw.x_y(loc.x, loc.y)
        .ellipse()
        .x_y(0.32 * r, 0.0)
        .w_h(0.3 * r, 0.3 * r)
        .color(ic);
    draw.x_y(loc.x, loc.y)
        .rotate((1.0 / 3.0) * f32::PI())
        .ellipse()
        .x_y(0.32 * r, 0.0)
        .w_h(0.3 * r, 0.3 * r)
        .color(ic);
    draw.x_y(loc.x, loc.y)
        .rotate((2.0 / 3.0) * f32::PI())
        .ellipse()
        .x_y(0.32 * r, 0.0)
        .w_h(0.3 * r, 0.3 * r)
        .color(ic);
    draw.x_y(loc.x, loc.y)
        .rotate(f32::PI())
        .ellipse()
        .x_y(0.32 * r, 0.0)
        .w_h(0.3 * r, 0.3 * r)
        .color(ic);
    draw.x_y(loc.x, loc.y)
        .rotate((4.0 / 3.0) * f32::PI())
        .ellipse()
        .x_y(0.32 * r, 0.0)
        .w_h(0.3 * r, 0.3 * r)
        .color(ic);
    draw.x_y(loc.x, loc.y)
        .rotate((5.0 / 3.0) * f32::PI())
        .ellipse()
        .x_y(0.32 * r, 0.0)
        .w_h(0.3 * r, 0.3 * r)
        .color(ic);
}
