use legion::systems::{Builder, CommandBuffer};
use legion::*;

use super::super::components::*;
use super::SystemBundle;
use crate::app::resource::Time;

fn ease_out_cubic(t: f32) -> f32 {
    (t - 1.0) * (t - 1.0) * (t - 1.0) + 1.0
}

#[system(for_each)]
pub fn enemy_damage(
    chip: &mut EnemyDamageChip,
    pos: &mut Position,
    entity: &Entity,
    #[resource] time: &Time,
    commands: &mut CommandBuffer,
) {
    chip.timer += time.delta;
    if chip.timer >= 1.5 {
        commands.remove(*entity);
    }
    pos.y = 300.0 - 50.0 * ease_out_cubic((chip.timer as f32).min(1.0));
}

#[system(for_each)]
pub fn player_damage(
    chip: &mut PlayerDamageChip,
    pos: &mut Position,
    entity: &Entity,
    #[resource] time: &Time,
    commands: &mut CommandBuffer,
) {
    chip.timer += time.delta;
    if chip.timer >= 1.5 {
        commands.remove(*entity);
    }
    pos.y = 120.0 - 50.0 * ease_out_cubic((chip.timer as f32).min(1.0));
}

pub struct DamageEffectSystemBundle;
impl SystemBundle for DamageEffectSystemBundle {
    fn build(schedule: &mut Builder) {
        schedule.add_system(enemy_damage_system());
        schedule.add_system(player_damage_system());
    }
}
