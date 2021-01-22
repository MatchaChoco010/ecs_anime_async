use ggez::graphics::Color;

pub struct Name {
    pub name: String,
}

pub enum Renderable {
    Circle {
        radius: f32,
        color: Color,
    },
    Rectangle {
        width: f32,
        height: f32,
        color: Color,
    },
}

pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct AnimationPlayer {
    pub anim_name: String,
    pub seconds: f64,
    pub frame: u64,
}
impl AnimationPlayer {
    pub fn new<S: ToString>(anim_name: S) -> Self {
        Self {
            anim_name: anim_name.to_string(),
            seconds: 0.0,
            frame: 0,
        }
    }
}
