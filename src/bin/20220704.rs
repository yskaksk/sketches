use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(1000, 1000).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let xn = 13;
    let r = win.w() / (xn as f32 * 1.5);
    let yn = (win.h() / (3.0.sqrt() * r)).ceil() as usize;
    if frame.nth() == 0 {
        draw.background().color(BEIGE);
        for i in 0..=xn {
            for j in 0..=yn {
                let x = 1.5 * r * i as f32 - win.w() * 0.5;
                let y = 3.0.sqrt() * r * j as f32 - win.h() * 0.5;
                let depth = ((x * x + y * y).sin() * 2.0 + 3.0).floor() as usize;
                draw_shape(&draw, pt2(x, y), r, depth);
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

fn draw_shape(draw: &Draw, loc: Point2, size: f32, depth: usize) {
    let color = [
        INDIANRED,
        CADETBLUE,
        DARKBLUE,
        DARKOLIVEGREEN,
        DODGERBLUE,
        LIGHTSALMON,
        DARKSALMON,
    ][depth];
    let theta = f32::PI() / 3.0;
    let points = (0..7).map(|i| {
        let x = size * ((i % 6) as f32 * theta).cos();
        let y = size * ((i % 6) as f32 * theta).sin();
        pt2(x, y)
    });
    //draw.xy(loc).polyline().points(points);
    draw.xy(loc).polygon().points(points).color(color);
    if depth > 0 {
        let r = size * 0.5;
        let s = size * 0.5;
        draw_shape(
            draw,
            pt2(loc.x + r * theta.cos(), loc.y + r * theta.sin()),
            s,
            depth - 1,
        );
        draw_shape(
            draw,
            pt2(
                loc.x + r * (3.0 * theta).cos(),
                loc.y + r * (3.0 * theta).sin(),
            ),
            s,
            depth - 1,
        );
        draw_shape(
            draw,
            pt2(
                loc.x + r * (5.0 * theta).cos(),
                loc.y + r * (5.0 * theta).sin(),
            ),
            s,
            depth - 1,
        );
    }
}
