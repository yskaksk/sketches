use nannou::color::encoding::Srgb;
use nannou::color::rgb::Rgb;
use nannou::prelude::*;

use sketches::util::gif_save_path;

fn main() {
    nannou::app(model).update(update).run()
}

struct Model {
    cells: Vec<Vec<i32>>,
}

impl Model {
    fn get_cell(&self, x: i32, y: i32) -> i32 {
        self.cells[((x + 10) % 10) as usize][((y + 10) % 10) as usize]
    }

    fn sum_neighbor(&self, x: i32, y: i32) -> i32 {
        self.get_cell(x - 1, y - 1)
            + self.get_cell(x - 1, y)
            + self.get_cell(x - 1, y + 1)
            + self.get_cell(x, y - 1)
            + self.get_cell(x, y + 1)
            + self.get_cell(x + 1, y - 1)
            + self.get_cell(x + 1, y)
            + self.get_cell(x + 1, y + 1)
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .view(view)
        .size(1000, 1000)
        .build()
        .unwrap();
    let cells = Vec::from_iter((0..10).map(|i| {
        Vec::from_iter((0..10).map(|j| {
            if ((i == 5) && (j == 4))
                || ((i == 6) && (j == 4))
                || ((i == 4) && (j == 5))
                || ((i == 5) && (j == 5))
                || ((i == 5) && (j == 6))
            {
                1
            } else {
                0
            }
        }))
    }));
    Model { cells }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let win = app.window_rect();
    let n = 10;
    draw.background().color(BEIGE);
    for i in 0..(n + 1) {
        let x = map_range(i, 0, n, win.left() * 0.9, win.right() * 0.9);
        let y = map_range(i, 0, n, win.bottom() * 0.9, win.top() * 0.9);
        draw_line(
            &draw,
            pt2(x, win.bottom() * 0.9),
            pt2(x, win.top() * 0.9),
            3.0,
            pt2(0.0, 0.0),
            rgb8(10, 10, 10),
        );
        draw_line(
            &draw,
            pt2(x, win.bottom() * 0.9),
            pt2(x, win.top() * 0.9),
            3.0,
            pt2(0.0, 0.0),
            rgb8(10, 10, 10),
        );
        draw_line(
            &draw,
            pt2(win.left() * 0.9, y),
            pt2(win.right() * 0.9, y),
            3.0,
            pt2(0.0, 0.0),
            rgb8(10, 10, 10),
        );
        draw_line(
            &draw,
            pt2(win.left() * 0.9, y),
            pt2(win.right() * 0.9, y),
            3.0,
            pt2(0.0, 0.0),
            rgb8(10, 10, 10),
        );
    }
    let d = win.w() * 0.9 / (n as f32);
    for i in 0..10 {
        for j in 0..10 {
            if model.cells[i][j] == 1 {
                let x = d * 0.5 + i as f32 * d - win.w() * 0.5 * 0.9;
                let y = d * 0.5 + j as f32 * d - win.h() * 0.5 * 0.9;
                fill_square(&draw, pt2(x, y), d, rgb8(15, 15, 15));
            }
        }
    }
    draw.to_frame(app, &frame).unwrap();
    if frame.nth() < 150 {
        let path = gif_save_path(app, frame.nth() as usize);
        app.main_window().capture_frame(&path);
    }
}

fn update(_: &App, model: &mut Model, _: Update) {
    let mut new_cells = vec![vec![0; 10]; 10];
    for i in 0..10 {
        for j in 0..10 {
            let s = model.sum_neighbor(i, j);
            if s <= 1 {
                new_cells[i as usize][j as usize] = 0;
            } else if s == 2 {
                new_cells[i as usize][j as usize] = model.get_cell(i, j);
            } else if s == 3 {
                new_cells[i as usize][j as usize] = 1;
            } else {
                new_cells[i as usize][j as usize] = 0;
            }
        }
    }
    model.cells = new_cells;
}

fn draw_line(
    draw: &Draw,
    start: Point2,
    end: Point2,
    weight: f32,
    loc: Point2,
    color: Rgb<Srgb, u8>,
) {
    let distance = start.distance(end);
    let n = ((distance / 20.0).ceil() as i32).max(2);
    let points = Vec::from_iter((0..n).map(|i| {
        let x = map_range(i, 0, n - 1, start.x, end.x) + random_range(-1.0, 1.0) * 3.0;
        let y = map_range(i, 0, n - 1, start.y, end.y) + random_range(-1.0, 1.0) * 3.0;
        pt2(x, y)
    }));
    draw.xy(loc)
        .polyline()
        .weight(weight)
        .points(points)
        .color(color);
}

fn fill_square(draw: &Draw, loc: Point2, size: f32, color: Rgb<Srgb, u8>) {
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
        draw_line(draw, s, *p, 3.0, loc, color);
        draw_line(draw, s, *p, 3.0, loc, color);
        s = *p;
    }
}
