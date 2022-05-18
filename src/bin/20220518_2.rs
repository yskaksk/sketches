use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::*;

const N: usize = 40;

fn main() {
    nannou::sketch(view).size(720, 720).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();

    draw.background().color(BLACK);
    let d = win.h() / (N as f32);
    let perlin = Perlin::new();
    let t = app.time / 5.0;

    for i in 0..N {
        for j in 0..N {
            let x = map_range(i, 0, N, win.left(), win.right());
            let y = map_range(j, 0, N, win.bottom(), win.top());
            let h = perlin.get([
                map_range(i, 0, N, 0.0, 1.0) as f64,
                map_range(j, 0, N, 0.0, 1.0) as f64,
                t as f64,
            ]) as f32;
            if h < 0.3 {
                draw.tri()
                    .points(
                        Point2::new(x, y),
                        Point2::new(x + d, y),
                        Point2::new(x, y + d),
                    )
                    .color(hsv(h, 1.0, 1.0));
            } else {
                draw.ellipse()
                    .x_y(x + d / 2.0, y + d / 2.0)
                    .color(hsva(h, 1.0, 1.0, 0.3));
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
