use anyhow::Result;
use futures::{join, pin_mut, select, FutureExt};
use ggez::event::KeyCode;
use ggez::graphics::Color;

mod app;
use app::components::*;
use app::*;

fn load_entities() {
    app::push((
        Name {
            name: "modal-window".to_string(),
        },
        Renderable::Rectangle {
            width: 400.0,
            height: 150.0,
            color: Color::from_rgba(0, 0, 0, 0),
        },
        Position {
            x: 400.0,
            y: 300.0,
            z: 10.0,
        },
    ));
    app::push((
        Name {
            name: "z-text".to_string(),
        },
        Renderable::Text {
            text: "OK (Z key)".to_string(),
            color: Color::from_rgba(255, 255, 255, 0),
        },
        Position {
            x: 250.0,
            y: 300.0,
            z: 15.0,
        },
    ));
    app::push((
        Name {
            name: "x-text".to_string(),
        },
        Renderable::Text {
            text: "Cancel (X key)".to_string(),
            color: Color::from_rgba(255, 255, 255, 0),
        },
        Position {
            x: 450.0,
            y: 300.0,
            z: 15.0,
        },
    ));
    app::push((
        Name {
            name: "select-effect-circle".to_string(),
        },
        Renderable::Circle {
            radius: 40.0,
            color: Color::from_rgba(255, 255, 255, 0),
        },
        Position {
            x: 300.0,
            y: 300.0,
            z: 12.0,
        },
    ));
}

fn main() -> Result<()> {
    let mut app = App::new("app", "Orito Itsuki")?;

    load_entities();
    app::load_animation("modal-fade-in", "./modal-fade-in.json")?;
    app::load_animation("modal-fade-out", "./modal-fade-out.json")?;
    app::load_animation("x-select-effect", "./x-select-effect.json")?;
    app::load_animation("z-select-effect", "./z-select-effect.json")?;

    runtime::spawn(async {
        loop {
            app::key_press(KeyCode::Z).await;
            app::play_animation("modal-fade-in").await;

            let press_z = app::key_press(KeyCode::Z).fuse();
            let press_x = app::key_press(KeyCode::X).fuse();
            pin_mut!(press_z);
            pin_mut!(press_x);

            select! {
                _ = press_x => {
                    join!(
                        app::play_animation("modal-fade-out"),
                        app::play_animation("x-select-effect")
                    );
                    println!("X pressed");
                },
                _ = press_z => {
                    join!(
                        app::play_animation("modal-fade-out"),
                        app::play_animation("z-select-effect")
                    );
                    println!("Z pressed");
                },
            }
        }
    });

    app.run()
}
