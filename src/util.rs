use nannou::prelude::*;

pub fn random_x(win: Rect<f32>) -> f32 {
    random_range(win.left(), win.right())
}

pub fn random_y(win: Rect<f32>) -> f32 {
    random_range(win.bottom(), win.top())
}

pub fn random_point(win: Rect<f32>) -> Point2 {
    pt2(random_x(win), random_y(win))
}

pub fn map_range_win(win: Rect<f32>, i: usize, j: usize, n: usize) -> Point2 {
    pt2(
        map_range(i, 0, n - 1, win.left(), win.right()),
        map_range(j, 0, n - 1, win.bottom(), win.top()),
    )
}

pub fn random_from_circle() -> Point2 {
    let theta = 2.0 * f32::PI() * random::<f32>();
    let r = random::<f32>().sqrt();
    return pt2(r * theta.cos(), r * theta.sin());
}

pub fn gif_save_path(app: &App, count: usize) -> String {
    format!("output/{}/image_{:>03}.png", app.exe_name().unwrap(), count)
}

pub fn cyclical_geom(val: f32, min: f32, max: f32) -> f32 {
    let d = max - min;
    assert!(d > 0.0);
    if val > max {
        return val - d;
    }
    if val < min {
        return val + d;
    }
    return val;
}

pub fn cyclical_rect(loc: Point2, win: Rect<f32>) -> Point2 {
    return pt2(
        cyclical_geom(loc.x, win.left(), win.right()),
        cyclical_geom(loc.y, win.bottom(), win.top()),
    );
}

pub fn curve_vertex(draw: &Draw, p0: Point2, p1: Point2, p2: Point2, p3: Point2) {
    let a4 = p1;
    let a3 = (p2 - p0) / 2.0;
    let a1 = (p3 - p1) / 2.0 - 2.0 * p2 + a3 + 2.0 * a4;
    let a2 = 3.0 * p2 - (p3 - p1) / 2.0 - 2.0 * a3 - 3.0 * a4;
    let points = Vec::from_iter((0..=100).map(|i| {
        let t = map_range(i, 0, 100, 0.0, 1.0);
        a1 * t.powi(3) + a2 * t.powi(2) + a3 * t + a4
    }));
    draw.polyline().points(points);
}

pub fn curve(draw: &Draw, points: Vec<Point2>) {
    assert!(points.len() >= 4, "at least 4 points needed");
    let mut p = points.clone();
    let head = points[0].clone();
    let tail = points[points.len() - 1].clone();
    p.insert(0, head);
    p.push(tail);
    for pp in p.windows(4) {
        curve_vertex(draw, pp[0], pp[1], pp[2], pp[3]);
    }
}

pub fn scribble_line(draw: &Draw, start: Point2, end: Point2) {
    let bowing = 1.0;
    let max_offset = 2.0;

    let lin_sq = start.distance_squared(end);
    let offset = (lin_sq.sqrt() / 10.0).min(max_offset);

    let half_offset = offset / 2.0;
    let diverge_point = random_range(0.2, 0.4);

    let _mid_disp_x = bowing * (end.y - start.y) / 200.0;
    let _mid_disp_y = bowing * (start.x - end.x) / 200.0;
    let mid_disp_x = get_offset(-_mid_disp_x, _mid_disp_x);
    let mid_disp_y = get_offset(-_mid_disp_y, _mid_disp_y);
    draw_scribble_line(
        draw,
        start,
        end,
        offset,
        mid_disp_x,
        mid_disp_y,
        diverge_point,
    );
    draw_scribble_line(
        draw,
        start,
        end,
        half_offset,
        mid_disp_x,
        mid_disp_y,
        diverge_point,
    );
}

fn draw_scribble_line(
    draw: &Draw,
    start: Point2,
    end: Point2,
    offset: f32,
    mid_disp_x: f32,
    mid_disp_y: f32,
    diverge_point: f32,
) {
    let p1 = pt2(
        start.x + get_offset(-offset, offset),
        start.y + get_offset(-offset, offset),
    );
    let p2 = pt2(
        mid_disp_x + start.x + (end.x - start.x) * diverge_point + get_offset(-offset, offset),
        mid_disp_y + start.y + (end.y - start.y) * diverge_point + get_offset(-offset, offset),
    );
    let p3 = pt2(
        mid_disp_x
            + start.x
            + 2.0 * (end.x - start.x) * diverge_point
            + get_offset(-offset, offset),
        mid_disp_y
            + start.y
            + 2.0 * (end.y - start.y) * diverge_point
            + get_offset(-offset, offset),
    );
    let p4 = pt2(
        end.x + get_offset(-offset, offset),
        end.y + get_offset(-offset, offset),
    );
    curve(draw, vec![p1, p2, p3, p4]);
}

fn get_offset(min_val: f32, max_val: f32) -> f32 {
    return random::<f32>() * (max_val - min_val) + min_val
}
