use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(1000, 1000).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let n = 5;
    let d = win.w() / n as f32;
    if frame.nth() == 0 {
        draw.background().color(WHITE);
        for i in 0..n {
            for j in 0..n {
                let x = d * 0.5 + d * i as f32 - win.w() * 0.5;
                let y = d * 0.5 + d * j as f32 - win.h() * 0.5;
                draw_shape(&draw, pt2(x, y), d * 0.93);
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

fn draw_shape(draw: &Draw, loc: Point2, size: f32) {
    let n = 10;
    let c = rgb8(163, 114, 42);
    for i in 0..n {
        if i == 0 || random::<f32>() > 0.03 {
            let d = map_range(i, 0, n-1, size, 20.0);
            let r = d * 0.5 * 2.0.sqrt();
            if random::<f32>() > 0.03 {
                let points = Vec::from_iter((0..=4).map(|i| {
                    let theta = f32::PI() * 0.5 * i as f32;
                    let x = r * theta.cos();
                    let y = r * theta.sin();
                    pt2(x, y)
                }));
                draw.x_y(loc.x, loc.y).rotate(f32::PI() / 4.0).polyline().weight(3.0).points(points).color(c);
            } else {
                for i in 0..4 {
                    let points = Vec::from_iter((0..=90).map(|i| {
                        let x = map_range(i % 90, 0, 89, r, 0.0) + random_range(-d*0.01, d*0.01);
                        let y = r - x + random_range(-d*0.01, d*0.01);
                        pt2(x, y)
                    }));
                    draw.x_y(loc.x, loc.y).rotate(f32::PI() / 4.0 + f32::PI() * 0.5 * i as f32).polyline().weight(4.0).points(points).color(GREEN);
                }
            }
        }
    }
}
