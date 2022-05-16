use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(600, 600).run()
}

fn view(app: &App, frame: Frame) {
    let draw = app.draw();

    draw.background().color(BLACK);

    let win = app.window_rect();
    let w_d = (app.mouse.x - win.left()).max(1.0);
    let h_d = (app.mouse.y - win.bottom()).max(1.0);
    let xcount = (win.w() / w_d).ceil() as usize;
    let ycount = (win.h() / h_d).ceil() as usize;

    for i in 0..=xcount {
        for j in 0..=ycount {
            let x = map_range(i, 0, xcount, win.left(), win.right());
            let y = map_range(j, 0, ycount, win.bottom(), win.top());
            draw.rect().x_y(x, y).w_h(w_d, h_d).color(hsv(
                map_range(x, win.left(), win.right(), 0.0, 1.0),
                map_range(y, win.bottom(), win.top(), 0.0, 1.0),
                1.0,
            ));
        }
    }
    draw.to_frame(app, &frame).unwrap();
}
