use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(1000, 1000).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let xn = 5;
    let r = win.w() / (xn as f32 * 1.5);
    let yn = (win.h() / (3.0.sqrt() * r)).ceil() as usize;
    if frame.nth() == 0 {
        draw.background().color(BEIGE);
        for i in 0..=xn {
            for j in 0..=yn {
                let x = 1.5 * r * i as f32 - win.w() * 0.5;
                let y = 3.0.sqrt() * r * j as f32 - win.h() * 0.5;
                let n = ((x + y).sin() * 2.0 + 3.0).floor() as usize;
                let depth = 5;
                draw_shape(&draw, r, pt2(x, y), depth, n);
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn draw_shape(draw: &Draw, r: f32, loc: Point2, depth: usize, n: usize) {
    let color = [
        INDIANRED,
        DARKSALMON,
        LIGHTSALMON,
        CADETBLUE,
        DARKBLUE,
        DARKOLIVEGREEN,
        DODGERBLUE,
    ][depth % 7];
    let theta = f32::PI() / 3.0;
    let points = (0..6).map(|i| {
        let x = r * (i as f32 * theta).cos();
        let y = r * (i as f32 * theta).sin();
        pt2(x, y)
    });
    draw.xy(loc).polygon().points(points).color(color);
    if depth > 0 {
        let dir = ((depth + n) % 6) as f32 * theta;
        draw_shape(
            draw,
            2.0 * (r / 3.0),
            pt2(loc.x + (r / 3.0) * dir.cos(), loc.y + (r / 3.0) * dir.sin()),
            depth - 1,
            n,
        );
        for i in 2..=4 {
            let ddir = dir + i as f32 * theta;
            draw_shape(
                draw,
                r / 3.0,
                pt2(
                    loc.x + (2.0 * r / 3.0) * ddir.cos(),
                    loc.y + (2.0 * r / 3.0) * ddir.sin(),
                ),
                depth - 1,
                n,
            );
        }
    }
}
