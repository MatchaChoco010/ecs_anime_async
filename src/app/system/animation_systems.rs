use std::collections::HashMap;

use legion::*;
use legion::{systems::Builder, Resources, World};

use super::super::components::*;
use super::super::keyframe::Animation;
use super::SystemBundle;
use crate::app::app::Time;

#[system(par_for_each)]
pub fn animation_timer_update(
    player: &mut AnimationPlayer,
    #[resource] animations: &HashMap<String, Animation>,
    #[resource] time: &Time,
) {
    let anim = animations
        .get(&player.anim_name)
        .expect(&format!("animation not found: {}", player.anim_name));

    if player.seconds < anim.total_frame as f64 / anim.fps {
        player.seconds += time.delta;
    }
    player.frame = ((player.seconds * anim.fps) as u64).min(anim.total_frame);
}

#[system(par_for_each)]
pub fn position_animation(
    name: &Name,
    pos: &mut Position,
    player: &AnimationPlayer,
    #[resource] animations: &HashMap<String, Animation>,
) {
    let anim = animations
        .get(&player.anim_name)
        .expect(&format!("animation not found: {}", player.anim_name));

    let tracks = anim
        .tracks
        .iter()
        .filter(|t| t.bind_name == name.name && t.bind_property == "Position");

    for track in tracks {
        let keys = track
            .keyframes
            .iter()
            .zip(track.keyframes.iter().skip(1))
            .find(|(k1, k2)| k1.position <= player.frame && player.frame < k2.position);

        match keys {
            // トラックに含まれる全てのキーフレームより前 or 後
            None => {
                let first = track.keyframes.first().expect("expect keyframe");
                let last = track.keyframes.last().expect("expect keyframe");
                if player.frame < first.position {
                    pos.x = first.value.x;
                    pos.y = first.value.y;
                    pos.z = first.value.z;
                } else if last.position < player.frame {
                    pos.x = last.value.x;
                    pos.y = last.value.y;
                    pos.z = last.value.z;
                }
            }
            // key1とkey2の間
            Some((key1, key2)) => {
                let key1_seconds = key1.position as f64 / anim.fps;
                let key2_seconds = key2.position as f64 / anim.fps;
                let t = ((player.seconds - key1_seconds) / (key2_seconds - key1_seconds)) as f32;
                pos.x = key1.value.x * (1.0 - t) + key2.value.x * t;
                pos.y = key1.value.y * (1.0 - t) + key2.value.y * t;
                pos.z = key1.value.z * (1.0 - t) + key2.value.z * t;
            }
        }
    }
}

pub struct AnimationSystemBundle;
impl SystemBundle for AnimationSystemBundle {
    fn build(_world: &mut World, resources: &mut Resources, schedule: &mut Builder) {
        resources.insert(HashMap::<String, Animation>::new());

        schedule.add_system(animation_timer_update_system());
        schedule.flush();
        schedule.add_system(position_animation_system());
    }
}
