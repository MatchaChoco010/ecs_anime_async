use nalgebra as na;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Animation {
    pub name: String,
    pub total_frame: u64,
    pub fps: f64,
    pub tracks: Vec<Track>,
}
impl Animation {
    pub fn sort_keyframes(&mut self) {
        for track in self.tracks.iter_mut() {
            track.keyframes.sort_by_key(|k| k.position);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Track {
    pub bind_name: String,
    pub bind_property: String,
    pub name: String,
    pub keyframes: Vec<Keyframe>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Keyframe {
    pub position: u64,
    pub value: na::Point3<f32>,
}
