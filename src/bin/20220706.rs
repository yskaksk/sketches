use nannou::prelude::*;

use sketches::util::gif_save_path;

static WEIGHT: f32 = 5.0;

fn main() {
    nannou::sketch(view).size(1000, 1000).run();
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BEIGE);
    let win = app.window_rect();
    let time = app.time;
    let n = 4;
    let d = win.w() / n as f32;
    for i in 0..n {
        for j in 0..n {
            let x = win.left() + d * i as f32 + d * 0.5;
            let y = win.top() - d * j as f32 - d * 0.5;
            draw_cistercian(
                &draw,
                pt2(x, y),
                d * 0.85,
                (time * 5.0 + (i as f32).powi(2) + (j as f32).powi(2)).ceil() as u32,
            );
        }
    }
    draw.to_frame(app, &frame).unwrap();
    if frame.nth() < 300 {
        let path = gif_save_path(app, frame.nth() as usize);
        app.main_window().capture_frame(&path);
    }
}

struct DigitLine {
    start: Point2,
    end: Point2,
}

impl DigitLine {
    fn new(start: Point2, end: Point2) -> Self {
        DigitLine { start, end }
    }
    fn draw(&self, draw: &Draw, loc: Point2, place: u32) {
        if place == 1 {
            draw.xy(loc)
                .line()
                .points(self.start, self.end)
                .weight(WEIGHT);
        }
        if place == 2 {
            let start = pt2(-self.start.x, self.start.y);
            let end = pt2(-self.end.x, self.end.y);
            draw.xy(loc).line().points(start, end).weight(WEIGHT);
        }
        if place == 3 {
            let start = pt2(self.start.x, -self.start.y);
            let end = pt2(self.end.x, -self.end.y);
            draw.xy(loc).line().points(start, end).weight(WEIGHT);
        }
        if place == 4 {
            let start = pt2(-self.start.x, -self.start.y);
            let end = pt2(-self.end.x, -self.end.y);
            draw.xy(loc).line().points(start, end).weight(WEIGHT);
        }
    }
}

fn print_digit(draw: &Draw, loc: Point2, size: f32, digit: u32, place: u32) {
    let d = size / 3.0;
    let half_size = size * 0.5;

    let one = DigitLine::new(pt2(0.0, half_size), pt2(d, half_size));
    let two = DigitLine::new(pt2(0.0, d * 0.5), pt2(d, d * 0.5));
    let three = DigitLine::new(pt2(0.0, half_size), pt2(d, d * 0.5));
    let four = DigitLine::new(pt2(0.0, d * 0.5), pt2(d, half_size));
    let six = DigitLine::new(pt2(d, half_size), pt2(d, d * 0.5));

    match digit {
        1 => one.draw(&draw, loc, place),
        2 => two.draw(&draw, loc, place),
        3 => three.draw(&draw, loc, place),
        4 => four.draw(&draw, loc, place),
        5 => {
            one.draw(&draw, loc, place);
            four.draw(&draw, loc, place);
        }
        6 => six.draw(&draw, loc, place),
        7 => {
            one.draw(&draw, loc, place);
            six.draw(&draw, loc, place);
        }
        8 => {
            two.draw(&draw, loc, place);
            six.draw(&draw, loc, place);
        }
        9 => {
            one.draw(&draw, loc, place);
            two.draw(&draw, loc, place);
            six.draw(&draw, loc, place);
        }
        _ => {}
    }
}

fn draw_cistercian(draw: &Draw, loc: Point2, size: f32, n: u32) {
    let number = n % 10000;
    let digit1 = number % 10;
    let digit2 = ((number - digit1) / 10) % 10;
    let digit3 = ((number - digit1 - digit2 * 10) / 100) % 10;
    let digit4 = ((number - digit1 - digit2 * 10 - digit3 * 100) / 1000) % 10;
    let d = size * 0.5;
    draw.xy(loc)
        .line()
        .points(pt2(0.0, d), pt2(0.0, -d))
        .weight(WEIGHT);

    print_digit(draw, loc, size, digit1, 1);
    print_digit(draw, loc, size, digit2, 2);
    print_digit(draw, loc, size, digit3, 3);
    print_digit(draw, loc, size, digit4, 4);
}
