use anyhow::Result;
use ggez::graphics::Color;

mod app;
use app::components::*;
use app::*;

fn main() -> Result<()> {
    let mut app = App::new("app", "Orito Itsuki")?;

    app.load_animation("anim1", "./anim1.json")?;
    app.push((
        Name {
            name: "circle".to_string(),
        },
        Renderable::Circle {
            radius: 30.0,
            color: Color::from_rgb(255, 128, 128),
        },
        Position {
            x: 100.0,
            y: 100.0,
            z: 5.0,
        },
        AnimationPlayer::new("anim1"),
    ));
    app.push((
        Name {
            name: "rect".to_string(),
        },
        Renderable::Rectangle {
            width: 50.0,
            height: 30.0,
            color: Color::from_rgb(0, 128, 255),
        },
        Position {
            x: 600.0,
            y: 100.0,
            z: 10.0,
        },
        AnimationPlayer::new("anim1"),
    ));

    app.run()
}
