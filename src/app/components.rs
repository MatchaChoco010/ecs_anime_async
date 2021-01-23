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
