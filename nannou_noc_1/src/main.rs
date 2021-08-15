use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Mover {
    loc: Vec2,
    vel: Vec2,
}

impl Mover {
    fn new(loc: Vec2, vel: Vec2) -> Self {
        Mover {
            loc: loc,
            vel: vel,
        }
    }
}

struct Model {
    movers: Vec<Mover>,
}

fn model(app: &App) -> Model {
    let _window = app.new_window().size(640, 360).view(view).build().unwrap();
    let mut movers = Vec::new();
    let mover = Mover::new(vec2(0.0, 0.0), vec2(1.0, 1.0));

    movers.push(mover);

    Model {
        movers
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win = app.window_rect();

    model.movers.iter_mut().for_each(|mover| {
        mover.loc = mover.loc + mover.vel;
        if mover.loc.x > win.right() || mover.loc.x < win.left() {
            mover.vel.x = mover.vel.x * -1.0;
        }
        if mover.loc.y > win.top() || mover.loc.y < win.bottom() {
            mover.vel.y = mover.vel.y * -1.0;
        }
    });
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    model.movers.iter().for_each(|mover| {
        draw.ellipse()
            .xy(mover.loc)
            .radius(5.0)
            .color(BLACK);
    });

    draw.to_frame(app, &frame).unwrap();
}
