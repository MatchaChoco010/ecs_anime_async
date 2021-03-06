//! アニメーションを再生するためのSystem。
//! 現状の実装ではアニメーションのバインドする対象がない場合をチェックできないし、
//! アニメーションのバインドがかぶっている場合もチェックできない。

use std::collections::HashMap;

use legion::systems::Builder;
use legion::*;
use rayon::prelude::*;

use super::super::components::*;
use super::super::keyframe::*;
use super::SystemBundle;
use crate::app::app::RESOURCES;
use crate::app::resource::{AnimationPlayerContainer, Time};

#[system]
pub fn animation_timer_update(
    #[resource] players: &mut AnimationPlayerContainer,
    #[resource] animations: &HashMap<String, Animation>,
    #[resource] time: &Time,
) {
    players
        .container
        .par_iter_mut()
        .filter(|(_, p)| p.playing)
        .for_each(|(anim_name, player)| {
            let anim = animations
                .get(anim_name)
                .expect(&format!("animation not found: {}", anim_name));

            if player.seconds < anim.total_frame as f64 / anim.fps {
                player.seconds += time.delta;
            } else {
                player.playing = false;
            }
            player.frame = ((player.seconds * anim.fps) as u64).min(anim.total_frame);
        });
}

#[system(par_for_each)]
pub fn position_animation(
    name: &Name,
    pos: &mut Position,
    #[resource] players: &AnimationPlayerContainer,
    #[resource] animations: &HashMap<String, Animation>,
) {
    players
        .container
        .iter()
        .filter(|(_, p)| p.playing)
        .for_each(|(anim_name, player)| {
            let anim = animations
                .get(anim_name)
                .expect(&format!("animation not found: {}", anim_name));

            let tracks = anim.tracks.iter().filter(|t| t.bind_name() == &name.name);

            for track in tracks {
                match track {
                    Track::PositionTrack { keyframes, .. } => {
                        let keys =
                            keyframes
                                .iter()
                                .zip(keyframes.iter().skip(1))
                                .find(|(k1, k2)| {
                                    k1.position <= player.frame && player.frame < k2.position
                                });

                        match keys {
                            // トラックに含まれる全てのキーフレームより前 or 後
                            None => {
                                let first = keyframes.first().expect("expect keyframe");
                                let last = keyframes.last().expect("expect keyframe");
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
                                let t = ((player.seconds - key1_seconds)
                                    / (key2_seconds - key1_seconds))
                                    as f32;
                                pos.x = key1.value.x * (1.0 - t) + key2.value.x * t;
                                pos.y = key1.value.y * (1.0 - t) + key2.value.y * t;
                                pos.z = key1.value.z * (1.0 - t) + key2.value.z * t;
                            }
                        }
                    }
                    _ => (),
                }
            }
        });
}

#[system(par_for_each)]
pub fn renderable_color_animation(
    name: &Name,
    renderable: &mut Renderable,
    #[resource] players: &AnimationPlayerContainer,
    #[resource] animations: &HashMap<String, Animation>,
) {
    players
        .container
        .iter()
        .filter(|(_, p)| p.playing)
        .for_each(|(anim_name, player)| {
            let anim = animations
                .get(anim_name)
                .expect(&format!("animation not found: {}", anim_name));

            let tracks = anim.tracks.iter().filter(|t| t.bind_name() == &name.name);

            for track in tracks {
                match track {
                    Track::RenderableColorTrack { keyframes, .. } => {
                        let keys =
                            keyframes
                                .iter()
                                .zip(keyframes.iter().skip(1))
                                .find(|(k1, k2)| {
                                    k1.position <= player.frame && player.frame < k2.position
                                });

                        match keys {
                            // トラックに含まれる全てのキーフレームより前 or 後
                            None => {
                                let first = keyframes.first().expect("expect keyframe");
                                let last = keyframes.last().expect("expect keyframe");
                                if player.frame < first.position {
                                    match renderable {
                                        Renderable::Circle { color, .. } => {
                                            color.r = first.value.x;
                                            color.g = first.value.y;
                                            color.b = first.value.z;
                                            color.a = first.value.w;
                                        }
                                        Renderable::Rectangle { color, .. } => {
                                            color.r = first.value.x;
                                            color.g = first.value.y;
                                            color.b = first.value.z;
                                            color.a = first.value.w;
                                        }
                                        Renderable::Text { color, .. } => {
                                            color.r = first.value.x;
                                            color.g = first.value.y;
                                            color.b = first.value.z;
                                            color.a = first.value.w;
                                        }
                                    }
                                } else if last.position < player.frame {
                                    match renderable {
                                        Renderable::Circle { color, .. } => {
                                            color.r = last.value.x;
                                            color.g = last.value.y;
                                            color.b = last.value.z;
                                            color.a = last.value.w;
                                        }
                                        Renderable::Rectangle { color, .. } => {
                                            color.r = last.value.x;
                                            color.g = last.value.y;
                                            color.b = last.value.z;
                                            color.a = last.value.w;
                                        }
                                        Renderable::Text { color, .. } => {
                                            color.r = last.value.x;
                                            color.g = last.value.y;
                                            color.b = last.value.z;
                                            color.a = last.value.w;
                                        }
                                    }
                                }
                            }
                            // key1とkey2の間
                            Some((key1, key2)) => {
                                let key1_seconds = key1.position as f64 / anim.fps;
                                let key2_seconds = key2.position as f64 / anim.fps;
                                let t = ((player.seconds - key1_seconds)
                                    / (key2_seconds - key1_seconds))
                                    as f32;
                                let r = key1.value.x * (1.0 - t) + key2.value.x * t;
                                let g = key1.value.y * (1.0 - t) + key2.value.y * t;
                                let b = key1.value.z * (1.0 - t) + key2.value.z * t;
                                let a = key1.value.w * (1.0 - t) + key2.value.w * t;
                                match renderable {
                                    Renderable::Circle { color, .. } => {
                                        color.r = r;
                                        color.g = g;
                                        color.b = b;
                                        color.a = a;
                                    }
                                    Renderable::Rectangle { color, .. } => {
                                        color.r = r;
                                        color.g = g;
                                        color.b = b;
                                        color.a = a;
                                    }
                                    Renderable::Text { color, .. } => {
                                        color.r = r;
                                        color.g = g;
                                        color.b = b;
                                        color.a = a;
                                    }
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        });
}

pub struct AnimationSystemBundle;
impl SystemBundle for AnimationSystemBundle {
    fn build(schedule: &mut Builder) {
        RESOURCES.with(|resources| {
            let mut resources = resources.borrow_mut();
            resources.insert(HashMap::<String, Animation>::new());
            resources.insert(AnimationPlayerContainer::new());
        });

        schedule.add_system(animation_timer_update_system());
        schedule.flush();
        schedule.add_system(position_animation_system());
        schedule.add_system(renderable_color_animation_system());
    }
}
