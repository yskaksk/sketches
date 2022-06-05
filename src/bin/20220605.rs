use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(1000, 1000).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let win = app.window_rect();

    {
        let width = 100.0;
        let color = rgb(0.0, 0.4, 0.6);
        let n = (win.w() / width).ceil() as usize;
        let hh = win.h() + width * 4.0;
        for i in 0..=n {
            let x = i as f32 * width - win.w() / 2.0;
            let t = ((i + 1) as f32 * 100.0 * app.time) % hh - hh / 2.0 + width * 2.0;
            draw_hole(&draw, win, pt2(x, t), color, width);
        }
    }
    {
        let width = 200.0;
        let color = rgb(0.0, 0.2, 0.8);
        let n = (win.w() / width).ceil() as usize;
        let hh = win.h() + width * 4.0;
        for i in 0..=n {
            let x = i as f32 * width - win.w() / 2.0;
            let t = ((i + 1) as f32 * 100.0 * app.time) % hh - hh / 2.0 + width * 2.0;
            draw_hole(&draw, win, pt2(x, t), color, width);
        }
    }
    {
        let width = 250.0;
        let color = rgb(0.0, 0.0, 1.0);
        let n = (win.w() / width).ceil() as usize;
        let hh = win.h() + width * 4.0;
        for i in 0..=n {
            let x = i as f32 * width - win.w() / 2.0;
            let t = 100.0 * app.time % hh - hh / 2.0 + width * 2.0 + width * (i as f32).sin();
            draw_hole(&draw, win, pt2(x, t), color, width);
        }
    }
    draw.to_frame(app, &frame).unwrap();
}

fn draw_hole(draw: &Draw, win: Rect, loc: Point2, color: Rgb<f32>, width: f32) {
    {
        let mut points = Vec::from_iter((0..=180).map(|i| {
            let theta = i as f32 * f32::PI() / 180.0;
            let x = width * 0.5 * theta.cos();
            let y = width * 0.5 * theta.sin();
            pt2(x, y)
        }));
        points.push(pt2(-width / 2.0, win.top() + win.h()));
        points.push(pt2(width / 2.0, win.top() + win.h()));
        draw.x_y(loc.x, loc.y).polygon().points(points).color(color);
    }
    for k in 0..3 {
        let mut points = Vec::from_iter((0..=180).map(|i| {
            let theta = i as f32 * f32::PI() / 180.0 + f32::PI();
            let x = width * 0.5 * theta.cos();
            let y = width * 0.5 * theta.sin() - width * k as f32;
            pt2(x, y)
        }));
        let mut points2 = Vec::from_iter((0..=180).map(|i| {
            let theta = i as f32 * f32::PI() / 180.0;
            let x = width * 0.5 * theta.cos();
            let y = width * 0.5 * theta.sin() - width * (k + 1) as f32;
            pt2(x, y)
        }));
        points.append(&mut points2);
        draw.x_y(loc.x, loc.y).polygon().points(points).color(color);
    }
    {
        let mut points = Vec::from_iter((0..=180).map(|i| {
            let theta = i as f32 * f32::PI() / 180.0 + f32::PI();
            let x = width * 0.5 * theta.cos();
            let y = width * 0.5 * theta.sin() - width * 3.0;
            pt2(x, y)
        }));
        points.push(pt2(width / 2.0, win.bottom() - win.h() - width * 4.0));
        points.push(pt2(-width / 2.0, win.bottom() - win.h() - width * 4.0));
        draw.x_y(loc.x, loc.y).polygon().points(points).color(color);
    }
}
