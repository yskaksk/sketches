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
