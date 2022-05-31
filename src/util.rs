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
