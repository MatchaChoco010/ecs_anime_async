use std::collections::HashMap;

pub struct AnimationPlayerContainer {
    pub container: HashMap<String, AnimationPlayer>,
}
impl AnimationPlayerContainer {
    pub fn new() -> Self {
        Self {
            container: HashMap::new(),
        }
    }

    pub fn new_anim<S: ToString>(&mut self, anim_name: S) {
        self.container
            .insert(anim_name.to_string(), AnimationPlayer::new());
    }
}

pub struct AnimationPlayer {
    pub playing: bool,
    pub seconds: f64,
    pub frame: u64,
}
impl AnimationPlayer {
    pub fn new() -> Self {
        Self {
            playing: true,
            seconds: 0.0,
            frame: 0,
        }
    }
}
