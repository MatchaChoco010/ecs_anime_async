use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use ggez::event::{self, EventHandler, EventsLoop};
use ggez::graphics::Color;
use ggez::{Context, ContextBuilder, GameResult};
use legion::*;
use serde_json;

use super::components::*;
use super::keyframe::Animation;
use super::render::render;

struct GameState {
    world: World,
    schedule: Schedule,
    resources: Resources,
}
impl GameState {
    fn new() -> Self {
        let mut resources = Resources::default();
        let mut world = World::default();
        let schedule = Schedule::builder().build();

        resources.insert(HashMap::<String, Animation>::new());

        world.push((
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
        ));
        world.push((
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
        ));

        Self {
            world,
            schedule,
            resources,
        }
    }
}
impl EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.schedule.execute(&mut self.world, &mut self.resources);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        render(ctx, &mut self.world)?;
        Ok(())
    }
}

pub struct App {
    ctx: Context,
    event_loop: EventsLoop,
    game_state: GameState,
}
impl App {
    pub fn new(app_id: &str, author: &str) -> Result<Self> {
        let (ctx, event_loop) = ContextBuilder::new(app_id, author).build()?;
        let game_state = GameState::new();

        Ok(Self {
            ctx,
            event_loop,
            game_state,
        })
    }

    pub fn load_animation<S: ToString, P: AsRef<Path>>(
        &mut self,
        anim_name: S,
        path: P,
    ) -> Result<()> {
        let buf = BufReader::new(File::open(path)?);
        let mut anim: Animation = serde_json::from_reader(buf)?;
        anim.sort_keyframes();

        self.game_state
            .resources
            .get_mut::<HashMap<String, Animation>>()
            .expect("expect anim hash map")
            .insert(anim_name.to_string(), anim);

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        event::run(&mut self.ctx, &mut self.event_loop, &mut self.game_state)?;
        Ok(())
    }
}
