use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(700, 700).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(WHITE);
        let R = 300.0;
        for _ in 0..100000 {
            let theta = random_range(0.0, f32::PI() * 2.0);
            let r = random_range(0.0, 0.5 * R * R).sqrt();
            let x = r * theta.cos();
            let y = r * theta.sin();
            draw.ellipse().x_y(x, y).w_h(2.0, 2.0).color(BLACK);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
