use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::thread_local;
use std::time::{Duration, Instant};

use anyhow::Result;
use ggez::{
    event::{self, EventHandler, EventsLoop, KeyCode, KeyMods},
    graphics::Color,
};
use ggez::{timer, Context, ContextBuilder, GameResult};
use legion::*;
use legion::{storage::IntoComponentSource, systems::Builder};
use serde_json;

use super::components::*;
use super::keyframe::Animation;
use super::render::render;
use super::resource::{AnimationPlayerContainer, KeyInputHashMap, Time};
use super::runtime;
use super::system::*;

thread_local! {
    pub static WORLD: RefCell<World> = RefCell::new(World::default());
    pub static RESOURCES: RefCell<Resources> = RefCell::new(Resources::default());
}

struct GameStateBuilderObject {
    schedule_builder: Builder,
}
impl GameStateBuilderObject {
    fn build(mut self) -> GameState {
        GameState {
            schedule: self.schedule_builder.build(),
        }
    }

    fn add_bundle<B: SystemBundle>(mut self) -> Self {
        B::build(&mut self.schedule_builder);
        self
    }

    fn flush(mut self) -> Self {
        self.schedule_builder.flush();
        self
    }
}

struct GameState {
    schedule: Schedule,
}
impl GameState {
    fn new() -> GameStateBuilderObject {
        RESOURCES.with(|r| {
            let mut r = r.borrow_mut();
            r.insert(Time { delta: 0.0 });
            r.insert(KeyInputHashMap::new());
        });
        GameStateBuilderObject {
            schedule_builder: Schedule::builder(),
        }
    }
}
impl EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        RESOURCES.with(|r| {
            let r = r.borrow_mut();
            let mut time = r.get_mut::<Time>().expect("expect Time");
            time.delta = timer::delta(ctx).as_secs_f64();
        });

        runtime::runtime_update();

        WORLD.with(|w| {
            let world = &mut *w.borrow_mut();
            RESOURCES.with(|r| {
                let resources = &mut *r.borrow_mut();
                self.schedule.execute(world, resources);
            })
        });

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        render(ctx)?;
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        RESOURCES.with(|r| {
            let r = r.borrow_mut();
            let mut key_input_hashmap = r
                .get_mut::<KeyInputHashMap>()
                .expect("expect KeyInputHashMap");
            key_input_hashmap.set_down(keycode)
        });
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymods: KeyMods) {
        RESOURCES.with(|r| {
            let r = r.borrow_mut();
            let mut key_input_hashmap = r
                .get_mut::<KeyInputHashMap>()
                .expect("expect KeyInputHashMap");
            key_input_hashmap.set_up(keycode)
        });
    }

    fn focus_event(&mut self, _ctx: &mut Context, gained: bool) {
        if gained {
            RESOURCES.with(|r| {
                let r = r.borrow_mut();
                let mut key_input_hashmap = r
                    .get_mut::<KeyInputHashMap>()
                    .expect("expect KeyInputHashMap");
                key_input_hashmap.reset();
            });
        }
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
            .add_bundle::<MessageSystemBundle>()
            .add_bundle::<AnimationSystemBundle>()
            .flush()
            .build();

        Ok(Self {
            ctx,
            event_loop,
            game_state,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        event::run(&mut self.ctx, &mut self.event_loop, &mut self.game_state)?;
        Ok(())
    }
}

pub fn push<T>(components: T) -> Entity
where
    Option<T>: IntoComponentSource,
{
    WORLD.with(|w| w.borrow_mut().push(components))
}

pub fn load_animation<S: ToString, P: AsRef<Path>>(anim_name: S, path: P) -> Result<()> {
    let buf = BufReader::new(File::open(path)?);
    let mut anim: Animation = serde_json::from_reader(buf)?;
    anim.sort_keyframes();

    RESOURCES.with(|r| {
        r.borrow_mut()
            .get_mut::<HashMap<String, Animation>>()
            .expect("expect anim hash map")
            .insert(anim_name.to_string(), anim)
    });

    Ok(())
}

pub fn play_animation_forget<S: ToString>(anim_name: S) {
    if cfg!(debug_assertions) {
        if RESOURCES.with(|r| {
            r.borrow()
                .get::<HashMap<String, Animation>>()
                .expect("expect anim hash map")
                .get(&anim_name.to_string())
                .is_none()
        }) {
            panic!(format!("No such name animation: {}", anim_name.to_string()));
        }
    }

    RESOURCES.with(|r| {
        r.borrow_mut()
            .get_mut::<AnimationPlayerContainer>()
            .expect("expect animation player container")
            .new_anim(anim_name.to_string())
    });
}

pub fn is_key_pressed(keycode: KeyCode) -> bool {
    RESOURCES.with(|r| {
        let r = r.borrow();
        let key_input_hashmap = r.get::<KeyInputHashMap>().expect("expect KeyInputHashMap");
        key_input_hashmap.pressed(keycode)
    })
}

pub async fn key_press(keycode: KeyCode) {
    while !is_key_pressed(keycode) {
        runtime::next_frame().await;
    }
}

pub async fn play_animation<S: ToString>(anim_name: S) {
    let anim = RESOURCES.with(|r| {
        r.borrow()
            .get::<HashMap<String, Animation>>()
            .expect("expect anim hash map")
            .get(&anim_name.to_string())
            .unwrap()
            .clone()
    });

    let start = Instant::now();
    let anim_duration = Duration::from_secs_f64(anim.total_frame as f64 / anim.fps);

    play_animation_forget(anim_name);

    loop {
        let now = Instant::now();
        let duration = now.duration_since(start);

        if anim_duration < duration {
            break;
        }

        runtime::delay(anim_duration - duration).await;
    }
}

pub fn add_message<S: ToString>(message: S) {
    WORLD.with(|world| {
        let mut world = world.borrow_mut();
        world.push((
            Position {
                x: 100.0,
                y: 350.0,
                z: 25.0,
            },
            Renderable::Rectangle {
                width: 200.0,
                height: 25.0,
                color: Color::from_rgb(0, 0, 0),
            },
            Message { timer: 0.0 },
        ));
        world.push((
            Position {
                x: 30.0,
                y: 350.0,
                z: 30.0,
            },
            Renderable::Text {
                text: message.to_string(),
                color: Color::from_rgb(255, 255, 255),
            },
            MessageText { timer: 0.0 },
        ));
    })
}
