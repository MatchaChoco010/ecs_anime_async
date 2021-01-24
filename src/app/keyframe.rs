use nalgebra as na;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Animation {
    pub name: String,
    pub total_frame: u64,
    pub fps: f64,
    pub tracks: Vec<Track>,
}
impl Animation {
    pub fn sort_keyframes(&mut self) {
        for track in self.tracks.iter_mut() {
            track.sort_keyframes();
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Track {
    PositionTrack {
        bind_name: String,
        keyframes: Vec<PositionKeyframe>,
    },
    RenderableColorTrack {
        bind_name: String,
        keyframes: Vec<RenderableColorKeyframe>,
    },
}
impl Track {
    fn sort_keyframes(&mut self) {
        match self {
            Track::PositionTrack { keyframes, .. } => keyframes.sort_by_key(|k| k.position),
            Track::RenderableColorTrack { keyframes, .. } => keyframes.sort_by_key(|k| k.position),
        }
    }

    pub fn bind_name(&self) -> &String {
        match self {
            Track::PositionTrack { bind_name, .. } => bind_name,
            Track::RenderableColorTrack { bind_name, .. } => bind_name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PositionKeyframe {
    pub position: u64,
    pub value: na::Point3<f32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RenderableColorKeyframe {
    pub position: u64,
    pub value: na::Vector4<f32>,
}
