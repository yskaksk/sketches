use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(1000, 1000).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    if frame.nth() == 0 {
        draw.background().color(WHITE);
        let win = app.window_rect();
        let n = 100;
        let d = win.w() / n as f32;
        for i in 0..n {
            for j in 0..n {
                let x = d * i as f32 - win.w() * 0.5;
                let y = d * j as f32 - win.h() * 0.5;
                let lt = pt2(x, y);
                let rt = pt2(x + d, y);
                let lb = pt2(x, y + d);
                let rb = pt2(x + d, y + d);
                let points = [
                    pt2(rotate(&lt).x, lt.y),
                    pt2(rotate(&rt).x, rt.y),
                    pt2(rotate(&rb).x, rb.y),
                    pt2(rotate(&lb).x, lb.y),
                ];
                let cc = ((x * x + y * y).cos() * (x * x + y * y).sqrt() / 500.0)
                    .min(1.0)
                    .max(0.0);
                draw.polygon().points(points).color(rgb(cc, cc, 0.1));
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn rotate(p: &Point2) -> Point2 {
    let abs = (p.x.pow(2.0) + p.y.pow(2.0)).sqrt();
    p.rotate(6.0 * f32::PI() * ((1.0 - abs / 500.0).max(0.0)).powf(10.0))
}
