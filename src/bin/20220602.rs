use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(1000, 1000).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let w = win.w();
    let h = win.h();
    let n = 20;
    let size = w / (n as f32);
    let xd = size * 3.0.sqrt() / 2.0;
    let yd = size * 0.75;
    let xcount = (w / xd).ceil() as usize;
    let ycount = (h / yd).ceil() as usize;
    if frame.nth() == 0 {
        draw.background().color(WHITE);
        for i in 0..=xcount {
            for j in 0..=ycount {
                let mut x = xd * i as f32 - w / 2.0;
                if j % 2 == 0 {
                    x += xd / 2.0;
                }
                let y = yd * j as f32 - h / 2.0;
                let loc = pt2(x, y);
                draw_shape(&draw, loc, size);
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
    draw_inner_shape(draw, loc, size, 0.0);
    draw_inner_shape(draw, loc, size, (2.0 / 3.0) * f32::PI());
    draw_inner_shape(draw, loc, size, (4.0 / 3.0) * f32::PI());
}

fn draw_inner_shape(draw: &Draw, loc: Point2, size: f32, rot: f32) {
    let d = size / 2.0;
    let sq3 = 3.0.sqrt();
    let shape1 = vec![
        pt2(0.0, 0.0),
        pt2(d / (2.0 * sq3), d / 2.0),
        pt2(0.0, 0.75 * d),
        pt2(-d / (2.0 * sq3), d / 2.0),
        pt2(0.0, 0.0),
    ];
    draw.polygon()
        .color(BLUE)
        .points(to_draw_pt2(shape1, loc, rot));
    let shape2 = vec![
        pt2(d / (2.0 * sq3), d / 2.0),
        pt2(d / sq3, 0.0),
        pt2(d * sq3 / 2.0, d / 2.0),
        pt2(d / (2.0 * sq3), d / 2.0),
    ];
    draw.polygon()
        .color(GREEN)
        .points(to_draw_pt2(shape2, loc, rot));
    let shape3 = vec![
        pt2(0.0, 0.75 * d),
        pt2(d * sq3 / 8.0, 0.875 * d),
        pt2(0.0, d),
        pt2(-d * sq3 / 8.0, 0.875 * d),
        pt2(0.0, 0.75 * d),
    ];
    draw.polygon()
        .color(RED)
        .points(to_draw_pt2(shape3, loc, rot));
}

fn to_draw_pt2(points: Vec<Point2>, loc: Point2, rot: f32) -> Vec<Point2> {
    let cos = rot.cos();
    let sin = rot.sin();
    Vec::from_iter(points.iter().map(|pt| {
        let x = pt.x * cos - pt.y * sin + loc.x;
        let y = pt.x * sin + pt.y * cos + loc.y;
        pt2(x, y)
    }))
}
