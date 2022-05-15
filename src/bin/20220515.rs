use nannou::{
    noise::{NoiseFn, Perlin},
    prelude::*,
};

fn main() {
    nannou::sketch(view).size(600, 600).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    let white = rgba8(255, 247, 247, 255);
    let red = rgba8(242, 49, 127, 255);
    let green = rgba8(5, 68, 92, 255);
    let black = rgba8(4, 0, 0, 255);
    draw.background().color(black);

    let win = app.window_rect();
    let r = win.w() / 10.0;
    let perlin = Perlin::new();
    let t = app.time / 100.0;
    for i in 0..10 {
        for j in 0..10 {
            let x = map_range(i, 0, 9, win.left(), win.right());
            let y = map_range(j, 0, 9, win.bottom(), win.top());
            let p1 = perlin.get([x as f64, y as f64, t as f64]) as f32;
            let p2 = perlin.get([x as f64, y as f64, (t + 0.5) as f64]) as f32;
            let c = if p1 < 0.25 {
                white
            } else if p1 < 0.5 {
                red
            } else if p1 < 0.75 {
                green
            } else {
                black
            };
            let d = 0.8 * r * (p2 + 0.5).min(1.2).max(0.1);
            draw.rect().x_y(x, y).w_h(d, d).color(c);
        }
    }
    draw.to_frame(app, &frame).unwrap();
    if app.keys.down.contains(&Key::S) {
        let path = format!("output/{}.png", app.exe_name().unwrap());
        app.main_window().capture_frame(&path);
        println!("file: {} saved!", path);
    }
}
