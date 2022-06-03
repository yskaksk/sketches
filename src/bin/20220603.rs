use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(2000, 1000).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let w = win.w();
    let h = win.h();
    if frame.nth() == 0 {
        draw.background().color(rgb(0.2, 0.2, 0.8));
        {
            let xd = w * 0.2;
            let yd = w * 0.15;
            let xc = (w / xd).ceil() as usize;
            let yc = (h / yd).ceil() as usize;
            for i in 0..=xc {
                for j in 0..=yc {
                    let mut x = i as f32 * xd - w / 2.0;
                    let y = j as f32 * yd - h / 2.0 + i as f32 * xd / 10.0;
                    if j % 2 == 0 {
                        x += xd / 2.0;
                    }
                    circle(&draw, pt2(x, y), 300.0);
                }
            }
        }
        for _ in 0..10 {
            let sx = random_range(win.left(), win.right());
            let gx = random_range(win.left(), win.right());
            draw.line()
                .start(pt2(sx, win.top()))
                .end(pt2(gx, win.bottom()))
                .color(rgba(0.1, 0.9, 0.8, 0.9));
        }
        {
            let xd = w * 0.1;
            let yd = w * 0.1;
            let xc = (w / xd).ceil() as usize;
            let yc = (h / yd).ceil() as usize;
            for i in 0..=2 * xc {
                for j in 0..=2 * yc {
                    let x = i as f32 * xd - w / 2.0;
                    let y = j as f32 * yd - h / 2.0 - i as f32 * xd / 2.0;
                    draw.ellipse()
                        .x_y(x, y)
                        .w_h(30.0, 30.0)
                        .color(rgba(0.9, 0.8, 0.7, 0.9));
                }
            }
        }
        {
            let xd = w * 0.25;
            let yd = xd * 0.5 * 3.0.sqrt();
            let xc = (w / xd).ceil() as usize;
            let yc = (h / yd).ceil() as usize;
            for i in 0..=2 * xc {
                for j in 0..=2 * yc {
                    let mut x = i as f32 * xd - w / 2.0;
                    let y = j as f32 * yd - h / 2.0 + i as f32 * yd / 10.0;
                    if j % 2 == 0 {
                        x += xd / 2.0;
                    }
                    circle2(&draw, pt2(x, y), 280.0);
                }
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::S) {
        let path = format!("output/{}/image.png", app.exe_name().unwrap());
        app.main_window().capture_frame(&path);
        println!("file: {} saved!", path);
    }
}

fn circle(draw: &Draw, loc: Point2, r: f32) {
    let points = (0..=360).map(|i| {
        let theta = i as f32 * f32::PI() / 180.0;
        let x = r * theta.cos() + loc.x;
        let y = r * theta.sin() + loc.y;
        pt2(x, y)
    });
    draw.polyline()
        .weight(20.0)
        .points(points)
        .color(rgba(0.2, 0.2, 0.2, 1.0));
}

fn circle2(draw: &Draw, loc: Point2, r: f32) {
    let points1 = (0..=360).map(|i| {
        let theta = i as f32 * f32::PI() / 180.0;
        let x = r * theta.cos() + loc.x;
        let y = r * theta.sin() + loc.y;
        pt2(x, y)
    });
    let points2 = (0..=360).map(|i| {
        let theta = i as f32 * f32::PI() / 180.0;
        let x = 0.9 * r * theta.cos() + loc.x;
        let y = 0.9 * r * theta.sin() + loc.y;
        pt2(x, y)
    });
    let c = rgba(0.9, 0.9, 0.2, 0.3);
    draw.polyline().weight(3.0).points(points1).color(c);
    draw.polyline().weight(3.0).points(points2).color(c);
}
