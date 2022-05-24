use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

#[derive(Clone, Copy)]
struct Circle {
    center: Point2,
    radius: f32,
}

impl Circle {
    fn new(center: Point2, radius: f32) -> Self {
        Self { center, radius }
    }
    fn draw(&self, draw: &Draw) {
        let h = map_range(self.radius, 0.0, 300.0, 0.05, 0.95);
        let a = 1.0;
        let c = hsva(h, 1.0, 1.0, a);
        draw.ellipse()
            .w_h(2.0 * self.radius, 2.0 * self.radius)
            .x_y(self.center.x, self.center.y)
            .color(c);
    }
}

struct Model {
    circles: Vec<Circle>,
}

fn model(app: &App) -> Model {
    app.new_window()
        .view(view)
        .key_released(key_released)
        .size(1920, 1080)
        .build()
        .unwrap();
    Model {
        circles: vec![Circle::new(pt2(0.0, 0.0), 100.0)],
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    frame.clear(WHITE);
    for c in model.circles.iter() {
        c.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn update(app: &App, model: &mut Model, _: Update) {
    let win = app.window_rect();
    let mut counter = 0;
    loop {
        counter += 1;
        let x = random_range(win.left(), win.right());
        let y = random_range(win.bottom(), win.top());
        let mut closest = win.w() + win.h();
        let mut drawable = false;
        for c in model.circles.iter() {
            let d = c.center.distance(pt2(x, y)) - c.radius;
            if d < 0.0 {
                drawable = false;
                break;
            }
            if d < closest {
                closest = d;
                drawable = true;
            }
        }
        if drawable && closest > 50.0 && closest < 300.0 {
            model.circles.push(Circle::new(pt2(x, y), closest));
            break;
        }
        if counter > 1000 {
            break;
        }
    }
}

fn key_released(app: &App, _: &mut Model, key: Key) {
    match key {
        Key::S => {
            let path = format!("output/{}/image.png", app.exe_name().unwrap());
            app.main_window().capture_frame(&path);
            println!("file: {} saved!", path);
        }
        _ => {}
    }
}
