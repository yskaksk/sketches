use kd_tree::{self, KdPoint, KdTree};
use nannou::prelude::*;
use typenum::U2;

fn main() {
    nannou::app(model).update(update).view(view).run()
}

//#[derive(Clone, Copy)]
//struct Circle {
//    center: Point2,
//    radius: f32
//}
//
//impl Circle {
//    fn new(center: Point2, radius: f32) -> Self {
//        Self {
//            center, radius
//        }
//    }
//    fn draw(&self, draw: &Draw) {
//        draw.ellipse().w_h(2.0 * self.radius, 2.0 * self.radius).x_y(self.center.x, self.center.y).color(BLACK);
//    }
//}

//impl KdPoint for Circle {
//    type Scalar = f32;
//    type Dim = U2;
//    fn at(&self, k: usize) -> f32 {
//        match k {
//            0 => self.center.x,
//            1 => self.center.y,
//            _ => unreachable!()
//        }
//    }
//}

#[derive(Clone, Copy)]
struct Eye {
    center: Point2,
    radius: f32,
    direction: f32,
}

impl Eye {
    fn new(center: Point2, radius: f32, direction: f32) -> Self {
        Eye {
            center,
            radius,
            direction,
        }
    }
    fn draw(&self, draw: &Draw) {
        let scale = 0.6;
        let inner_x = self.center.x + scale * self.radius * self.direction.cos();
        let inner_y = self.center.y + scale * self.radius * self.direction.sin();
        draw.ellipse()
            .x_y(self.center.x, self.center.y)
            .w_h(2.0 * self.radius, 2.0 * self.radius)
            .color(BLACK);
        draw.ellipse()
            .x_y(inner_x, inner_y)
            .w_h(
                self.radius * (1.0 - scale) * 2.0,
                self.radius * (1.0 - scale) * 2.0,
            )
            .color(WHITE);
    }
}

impl KdPoint for Eye {
    type Scalar = f32;
    type Dim = U2;
    fn at(&self, k: usize) -> f32 {
        match k {
            0 => self.center.x,
            1 => self.center.y,
            _ => unreachable!(),
        }
    }
}

struct Model {
    tree: KdTree<Eye>,
}

fn model(app: &App) -> Model {
    app.new_window().size(600, 600).build().unwrap();
    let tree = KdTree::build_by_ordered_float(vec![Eye::new(pt2(0.0, 0.0), 15.0, 0.0)]);
    Model { tree }
}

fn update(app: &App, model: &mut Model, _: Update) {
    let win = app.window_rect();
    if model.tree.items().len() < 1000 {
        loop {
            let x = random_range(win.left(), win.right());
            let y = random_range(win.bottom(), win.top());
            let r = random_range::<f32>(3.0, 10.0);
            let nearest = model.tree.nearest(&Eye::new(pt2(x, y), r, 0.0)).unwrap();
            let closest_point = nearest.item;
            let d = nearest.squared_distance.sqrt() - closest_point.radius;
            if d > r {
                let theta = (closest_point.center.y - y).atan2(closest_point.center.x - x);
                let mut points = Vec::from_iter(model.tree.items().iter().map(|c| *c));
                points.push(Eye::new(
                    pt2(x + (d - r) * theta.cos(), y + (d - r) * theta.sin()),
                    r,
                    theta,
                ));
                model.tree = KdTree::build_by_ordered_float(points);
                break;
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);
    for o in model.tree.items().iter() {
        o.draw(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
