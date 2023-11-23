use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct ScrollPane {
    pub position: Vec2,
}

#[derive(Debug, Default, Component)]
pub struct TextInput {
    pub active: bool,
}
