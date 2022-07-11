use nannou::prelude::*;
use sketches::util::gif_save_path;

fn main() {
    nannou::sketch(view).size(1000, 1000).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let n = 10;
    if frame.nth() % 10 == 0 {
        draw.background().color(BEIGE);
        for i in 0..n {
            let x = map_range(i, 0, n - 1, win.left() * 0.9, win.right() * 0.9);
            let y = map_range(i, 0, n - 1, win.bottom() * 0.9, win.top() * 0.9);
            draw_line(
                &draw,
                pt2(win.left() * 0.9, y),
                pt2(win.right() * 0.9, y),
                pt2(0.0, 0.0),
            );
            draw_line(
                &draw,
                pt2(x, win.bottom() * 0.9),
                pt2(x, win.top() * 0.9),
                pt2(0.0, 0.0),
            );
        }
        let d = (win.w() * 0.9) / (n - 1) as f32;
        for i in 0..n - 1 {
            for j in 0..n - 1 {
                let x = d * 0.5 + d * i as f32 - win.w() * 0.9 * 0.5;
                let y = d * 0.5 + d * j as f32 - win.h() * 0.9 * 0.5;
                if (2.0 * f32::PI() * (x * x + y * y).sqrt() / 450.0 + app.time * 2.0).sin() < 0.0 {
                    fill_square(&draw, pt2(x, y), d * 0.95);
                }
            }
        }
    }
    if frame.nth() < 300 {
        let path = gif_save_path(app, frame.nth() as usize);
        app.main_window().capture_frame(&path);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn draw_line(draw: &Draw, start: Point2, end: Point2, loc: Point2) {
    let distance = start.distance(end);
    let n = ((distance / 20.0).ceil() as i32).max(2);
    let points = Vec::from_iter((0..n).map(|i| {
        let x = map_range(i, 0, n - 1, start.x, end.x) + random_range(-1.0, 1.0) * 3.0;
        let y = map_range(i, 0, n - 1, start.y, end.y) + random_range(-1.0, 1.0) * 3.0;
        pt2(x, y)
    }));
    draw.xy(loc)
        .polyline()
        .weight(5.0)
        .points(points)
        .color(rgb(0.3, 0.3, 0.3));
}

fn fill_square(draw: &Draw, loc: Point2, size: f32) {
    let tl = pt2(-size * 0.5, size * 0.5);
    let tr = pt2(size * 0.5, size * 0.5);
    let bl = pt2(-size * 0.5, -size * 0.5);
    let br = pt2(size * 0.5, -size * 0.5);
    let n = 5 as i32;
    let points = Vec::from_iter((1..4 * n).map(|i| {
        let p = if (i % 2 == 0) && (i < 2 * n - 1) {
            let x = map_range(i, 0, 2 * n, tl.x, bl.x);
            let y = map_range(i, 0, 2 * n, tl.y, bl.y);
            pt2(x, y)
        } else if (i % 2 == 1) && (i <= 2 * n - 1) {
            let x = map_range(i, -1, 2 * n - 1, tl.x, tr.x);
            let y = map_range(i, -1, 2 * n - 1, tl.y, tr.y);
            pt2(x, y)
        } else if (i % 2 == 0) && (i > 2 * n - 1) {
            let x = map_range(i, 2 * n, 4 * n, bl.x, br.x);
            let y = map_range(i, 2 * n, 4 * n, bl.y, br.y);
            pt2(x, y)
        } else {
            let x = map_range(i, 2 * n - 1, 4 * n - 1, tr.x, br.x);
            let y = map_range(i, 2 * n - 1, 4 * n - 1, tr.y, br.y);
            pt2(x, y)
        };
        p
    }));
    let mut s = tl;
    for p in points.iter() {
        draw_line(draw, s, *p, loc);
        s = *p;
    }
}
