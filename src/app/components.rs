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
    Text {
        text: String,
        color: Color,
    },
}

pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

pub struct Message {
    pub timer: f64,
}

pub struct MessageText {
    pub timer: f64,
}

pub struct EnemyDamageChip {
    pub timer: f64,
}

pub struct EnemyHpBar;

pub struct PlayerDamageChip {
    pub timer: f64,
}
pub struct PlayerHpBar;

pub struct PlayerHpText;

pub struct SubMenuHighlight {
    pub index: usize,
}

pub struct SubMenuText {
    pub index: usize,
}

pub struct SubMenuDescriptionText;
