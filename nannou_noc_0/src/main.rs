use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    x: f32,
    y: f32,
    xspeed: f32,
    yspeed: f32,
}

fn model(app: &App) -> Model {
    let x = 100.0;
    let y = 100.0;
    let xspeed = 2.5;
    let yspeed = 2.0;

    let _window = app.new_window().size(800, 200).view(view).build().unwrap();

    Model {
        x,
        y,
        xspeed,
        yspeed,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.x = model.x + model.xspeed;
    model.y = model.y + model.yspeed;


    let win = app.window_rect();

    if model.x > win.right() || model.x < win.left() {
        model.xspeed = model.xspeed * -1.0;
    }
    if model.y > win.top() || model.y < win.bottom() {
        model.yspeed = model.yspeed * -1.0;
    }

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.ellipse()
        .x_y(model.x, model.y)
        .w_h(50.0, 50.0)
        .rgba(0.5, 0.5, 0.5, 1.0);

    draw.to_frame(app, &frame).unwrap();
}
