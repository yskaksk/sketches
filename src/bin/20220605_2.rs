use nannou::prelude::*;

use sketches::util::gif_save_path;

fn main() {
    nannou::sketch(view).size(1000, 1000).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let t = app.time;
    draw.background().color(WHITE);

    {
        let size = 50.0;
        let sq3 = 3.0.sqrt();
        let rot = (1.0 / 6.0) * f32::PI();
        let c = rgb(0.9, 0.9, 0.9);
        let ncount = (win.w() / size).ceil() as usize;
        for i in 0..ncount {
            for j in 0..ncount {
                let x = i as f32 * sq3 * size - win.w() / 2.0;
                let mut y = j as f32 * 3.0 * size - win.h() / 2.0;
                let mut r = rot;
                if (i + j) % 2 == 0 {
                    y -= size;
                    r += f32::PI();
                }
                draw_tri(&draw, size, pt2(x, y), r, c);
            }
        }
    }
    {
        let size = 250.0;
        let sq3 = 3.0.sqrt();
        let rot = (1.0 / 6.0) * f32::PI();
        let c = rgb(0.5, 0.5, 0.5);
        let ncount = (win.w() / size).ceil() as usize;
        let xt = (200.0 * t) % win.w() - win.w();
        for i in 0..ncount * 2 {
            for j in 0..ncount {
                let x = i as f32 * sq3 * size - win.w() / 2.0 + xt;
                let mut y = j as f32 * 3.0 * size - win.h() / 2.0;
                let mut r = rot;
                if (i + j) % 2 == 0 {
                    y -= size;
                    r += f32::PI();
                }
                draw_tri(&draw, size, pt2(x, y), r, c);
            }
        }
    }
    {
        let size = 350.0;
        let sq3 = 3.0.sqrt();
        let rot = (1.0 / 6.0) * f32::PI();
        let c = rgb(0.1, 0.1, 0.1);
        let ncount = (win.w() / size).ceil() as usize;
        let xt = (100.0 * t) % win.w() - win.w();
        for i in 0..ncount * 2 {
            for j in 0..ncount {
                let x = i as f32 * sq3 * size - win.w() / 2.0 + xt;
                let mut y = j as f32 * 3.0 * size - win.h() / 2.0;
                let mut r = rot;
                if (i + j) % 2 == 0 {
                    y -= size;
                    r += f32::PI();
                }
                draw_tri(&draw, size, pt2(x, y), r, c);
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
    if frame.nth() < 300 {
        let path = gif_save_path(&app, frame.nth() as usize);
        app.main_window().capture_frame(&path);
    }
    if app.keys.down.contains(&Key::S) {
        let path = format!("output/{}/image.png", app.exe_name().unwrap());
        app.main_window().capture_frame(&path);
        println!("file: {} saved!", path);
    }
}

fn draw_tri(draw: &Draw, size: f32, loc: Point2, rot: f32, color: Rgb<f32>) {
    let mut points = Vec::from_iter((0..=60).map(|i| {
        let theta = map_range(i, 0, 60, 0.0, (2.0 / 3.0) * f32::PI()) - (1.0 / 3.0) * f32::PI();
        pt2(size * theta.cos(), size * theta.sin())
    }));
    points.push(pt2(2.0 * size, 0.0));
    draw.x_y(loc.x, loc.y)
        .rotate(rot)
        .polygon()
        .points(points.clone())
        .color(color);
    draw.x_y(loc.x, loc.y)
        .rotate(rot + (2.0 / 3.0) * f32::PI())
        .polygon()
        .points(points.clone())
        .color(color);
    draw.x_y(loc.x, loc.y)
        .rotate(rot + (4.0 / 3.0) * f32::PI())
        .polygon()
        .points(points.clone())
        .color(color);
}
