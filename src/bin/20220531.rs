use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(1000, 1000).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let c1 = rgba(0.0, 0.9, 0.2, 1.0);
    let c2 = rgba(0.9, 0.0, 0.7, 1.0);
    if frame.nth() % 100 == 0 {
        draw.background().color(WHITE);
        let win = app.window_rect();
        let w = win.w();
        let h = win.h();
        let nsplit = 4;
        for i in 0..nsplit {
            for j in 0..nsplit {
                let x = win.left() + w / (2.0 * nsplit as f32) + (i as f32) * w / (nsplit as f32);
                let y = win.bottom() + h / (2.0 * nsplit as f32) + (j as f32) * h / (nsplit as f32);
                if i == 1 && j == 1 {
                    draw_square_3(&draw, pt2(x, y), w / (nsplit as f32), c2);
                } else {
                    draw_square_3(&draw, pt2(x, y), w / (nsplit as f32), c1);
                }
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn draw_square_1(draw: &Draw, loc: Point2, size: f32) {
    let n = 5;
    let mut d = size;
    for i in 0..n {
        let c = if i % 2 == 0 { BLACK } else { YELLOW };
        draw.rect().x_y(loc.x, loc.y).w_h(d, d).color(c);
        d -= size / (n as f32 + 1.0);
    }
}

fn draw_square_2(draw: &Draw, loc: Point2, size: f32) {
    let mut d = size;
    let mut center = loc;
    let n = 20;
    for i in 0..n {
        let c = if i % 2 == 0 {
            rgba(0.0, 0.0, 0.0, 1.0)
        } else {
            rgba(0.1, 0.1, 0.8, 0.9)
        };
        draw.rect().x_y(center.x, center.y).w_h(d, d).color(c);
        d -= size / (n as f32 + 1.0);
        let theta = 4.0 * i as f32 * f32::PI() / n as f32;
        center.x += (size / (4.0 * n as f32 + 1.0)) * theta.cos();
        center.y += size / (4.0 * n as f32 + 1.0) * theta.sin();
    }
}

fn draw_square_3(draw: &Draw, loc: Point2, size: f32, color: Rgba) {
    let center = loc;
    let left = loc.x - size / 2.0;
    let right = loc.x + size / 2.0;
    let top = loc.y + size / 2.0;
    let bottom = loc.y - size / 2.0;
    for i in 0..100 {
        let x = random_range(center.x - size / 2.0, center.x + size / 2.0);
        let y = random_range(center.y - size / 2.0, center.y + size / 2.0);
        let r = vec![x - left, right - x, top - y, y - bottom]
            .iter()
            .fold(0.0 / 0.0, |m, v| v.min(m))
            .min(size / 5.0);
        let c = if i % 2 == 0 {
            //rgba(0.0, 0.9, 0.2, 1.0)
            color
        } else {
            rgba(0.2, 0.1, 0.1, 1.0)
        };
        draw.ellipse().x_y(x, y).w_h(r, r).color(c);
    }
}
