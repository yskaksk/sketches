use nannou::prelude::*;
use nannou_touchosc::TouchOscClient;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    touchosc: TouchOscClient,
}

fn model(app: &App) -> Model {
    app.new_window().size(600, 600).view(view).build().unwrap();

    let mut touchosc = TouchOscClient::new(6555);
    touchosc.verbose();

    touchosc.add_fader("/color_r", 0.0, 1.0, 0.5);
    touchosc.add_fader("/color_g", 0.0, 1.0, 0.5);
    touchosc.add_fader("/color_b", 0.0, 1.0, 0.5);
    touchosc.add_fader("/color_a", 0.0, 1.0, 0.5);

    Model { touchosc }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    let c = rgba(
        model.touchosc.fader("/color_r"),
        model.touchosc.fader("/color_g"),
        model.touchosc.fader("/color_b"),
        model.touchosc.fader("/color_a"),
    );
    draw.ellipse().x_y(0.0, 0.0).w_h(400.0, 400.0).color(c);
    draw.to_frame(app, &frame).unwrap();
}

fn update(_: &App, model: &mut Model, _: Update) {
    model.touchosc.update();
}
