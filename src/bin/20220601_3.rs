use nannou::prelude::*;

use sketches::util;

fn main() {
    nannou::sketch(view).size(1000, 1000).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let w = win.w();
    draw.background().color(WHITE);

    let n = 10;
    let size = w / (n - 1) as f32;
    let half = size / 2.0;
    let sq3_2 = (3.0).sqrt() / 2.0;
    for i in 0..=n {
        for j in 0..=n {
            let x = half + (i - 1) as f32 * size - w / 2.0;
            let y = j as f32 * sq3_2 * size - w / 2.0;
            let mut loc = pt2(x, y);
            if j % 2 == 0 {
                loc.x -= half;
            }
            draw_shape(&draw, size, loc);
        }
    }

    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::S) {
        let path = format!("output/{}/image.png", app.exe_name().unwrap());
        app.main_window().capture_frame(&path);
        println!("file: {} saved!", path);
    }
}

fn draw_shape(draw: &Draw, size: f32, loc: Point2) {
    let weight = 2.0;
    let d = size / 3.0;
    let one_third = f32::PI() / 3.0;

    let mut start = loc;
    let mut end = pt2(loc.x + d, loc.y);
    draw.line().start(start).end(end).weight(weight);

    start = end;
    end.x += d * (2.0 * one_third).cos();
    end.y += d * (2.0 * one_third).sin();
    draw.line().start(start).end(end).weight(weight);

    start = end;
    end.x += d;
    draw.line().start(start).end(end).weight(weight);

    start = end;
    end.x += d * (5.0 * one_third).cos();
    end.y += d * (5.0 * one_third).sin();
    draw.line().start(start).end(end).weight(weight);

    start = end;
    end.x += d * one_third.cos();
    end.y += d * one_third.sin();
    draw.line().start(start).end(end).weight(weight);

    start = end;
    end.x += d * (5.0 * one_third).cos();
    end.y += d * (5.0 * one_third).sin();
    draw.line().start(start).end(end).weight(weight);

    start = end;
    end.x += d * (4.0 * one_third).cos();
    end.y += d * (4.0 * one_third).sin();
    draw.line().start(start).end(end).weight(weight);

    start = end;
    end.x += d;
    draw.line().start(start).end(end).weight(weight);

    start = end;
    end.x += d * (4.0 * one_third).cos();
    end.y += d * (4.0 * one_third).sin();
    draw.line().start(start).end(end).weight(weight);
}
