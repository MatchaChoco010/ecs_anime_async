use legion::systems::{Builder, CommandBuffer};
use legion::world::SubWorld;
use legion::*;

use super::super::components::*;
use super::SystemBundle;
use crate::app::resource::Time;

fn ease_in_out_cubic(t: f32) -> f32 {
    let t = t * 2.0;
    if t < 1.0 {
        0.5 * t * t * t
    } else {
        0.5 * ((t - 2.0) * (t - 2.0) * (t - 2.0) + 2.0)
    }
}

#[system]
#[write_component(Position)]
#[write_component(Renderable)]
#[write_component(Message)]
pub fn message(world: &mut SubWorld, #[resource] time: &Time, commands: &mut CommandBuffer) {
    let mut message_rectangles = <(Entity, &mut Message, &mut Position, &mut Renderable)>::query()
        .iter_mut(world)
        .collect::<Vec<_>>();
    for (&e, m, _p, _r) in message_rectangles.iter_mut() {
        m.timer += time.delta;
        if m.timer >= 4.0 {
            commands.remove(e);
        }
    }
    message_rectangles
        .sort_by(|(_, m1, _, _), (_, m2, _, _)| m1.timer.partial_cmp(&m2.timer).unwrap());
    let mut h = 0.0;
    for (_, m, p, r) in message_rectangles {
        p.y = 550.0 - h;
        let fade_in_alpha = (m.timer * 2.0).min(1.0);
        let fade_out_alpha = ((4.0 - m.timer) * 2.0).min(1.0);
        let alpha = fade_in_alpha * fade_out_alpha;
        match r {
            Renderable::Rectangle { color, .. } => {
                color.a = alpha as f32 * 0.85;
            }
            _ => (),
        }
        h += 30.0 * ease_in_out_cubic((m.timer as f32 * 2.0).min(1.0));
    }
}

#[system]
#[write_component(Position)]
#[write_component(Renderable)]
#[write_component(MessageText)]
pub fn message_text(world: &mut SubWorld, #[resource] time: &Time, commands: &mut CommandBuffer) {
    let mut message_rectangles =
        <(Entity, &mut MessageText, &mut Position, &mut Renderable)>::query()
            .iter_mut(world)
            .collect::<Vec<_>>();
    for (&e, m, _p, _r) in message_rectangles.iter_mut() {
        m.timer += time.delta;
        if m.timer >= 4.0 {
            commands.remove(e);
        }
    }
    message_rectangles
        .sort_by(|(_, m1, _, _), (_, m2, _, _)| m1.timer.partial_cmp(&m2.timer).unwrap());
    let mut h = 0.0;
    for (_, m, p, r) in message_rectangles {
        p.y = 550.0 - h - 10.0;
        let fade_in_alpha = (m.timer * 2.0 - 0.8).max(0.0).min(1.0);
        let fade_out_alpha = ((4.0 - m.timer) * 2.0).min(1.0);
        let alpha = fade_in_alpha * fade_out_alpha;
        match r {
            Renderable::Text { color, .. } => {
                color.a = alpha as f32;
            }
            _ => (),
        }
        h += 30.0 * ease_in_out_cubic((m.timer as f32 * 2.0).min(1.0));
    }
}

pub struct MessageSystemBundle;
impl SystemBundle for MessageSystemBundle {
    fn build(schedule: &mut Builder) {
        schedule.add_system(message_system());
        schedule.add_system(message_text_system());
    }
}
