use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use ggez::event::{self, EventHandler, EventsLoop};
use ggez::{timer, Context, ContextBuilder, GameResult};
use legion::*;
use legion::{storage::IntoComponentSource, systems::Builder};
use serde_json;

use super::keyframe::Animation;
use super::render::render;
use super::resource::{AnimationPlayerContainer, Time};
use super::system::*;

struct GameStateBuilderObject {
    world: World,
    resources: Resources,
    schedule_builder: Builder,
}
impl GameStateBuilderObject {
    fn build(mut self) -> GameState {
        GameState {
            world: self.world,
            resources: self.resources,
            schedule: self.schedule_builder.build(),
        }
    }

    fn add_bundle<B: SystemBundle>(mut self) -> Self {
        B::build(
            &mut self.world,
            &mut self.resources,
            &mut self.schedule_builder,
        );
        self
    }

    fn flush(mut self) -> Self {
        self.schedule_builder.flush();
        self
    }
}

struct GameState {
    world: World,
    resources: Resources,
    schedule: Schedule,
}
impl GameState {
    fn new() -> GameStateBuilderObject {
        let mut resources = Resources::default();
        resources.insert(Time { delta: 0.0 });
        GameStateBuilderObject {
            world: World::default(),
            resources,
            schedule_builder: Schedule::builder(),
        }
    }
}
impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        {
            let mut time = self.resources.get_mut::<Time>().expect("expect Time");
            time.delta = timer::delta(ctx).as_secs_f64();
        }

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

        let game_state = GameState::new()
            .add_bundle::<AnimationSystemBundle>()
            .flush()
            .build();

        Ok(Self {
            ctx,
            event_loop,
            game_state,
        })
    }

    pub fn push<T>(&mut self, components: T) -> Entity
    where
        Option<T>: IntoComponentSource,
    {
        self.game_state.world.push(components)
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

    pub fn play_animation<S: ToString>(&mut self, anim_name: S) {
        if cfg!(debug_assertions) {
            if self
                .game_state
                .resources
                .get::<HashMap<String, Animation>>()
                .expect("expect anim hash map")
                .get(&anim_name.to_string())
                .is_none()
            {
                panic!(format!("No such name animation: {}", anim_name.to_string()));
            }
        }

        self.game_state
            .resources
            .get_mut::<AnimationPlayerContainer>()
            .expect("expect animation player container")
            .new_anim(anim_name.to_string());
    }

    pub fn run(&mut self) -> Result<()> {
        event::run(&mut self.ctx, &mut self.event_loop, &mut self.game_state)?;
        Ok(())
    }
}
