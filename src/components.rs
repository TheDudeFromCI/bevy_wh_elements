use bevy::prelude::*;

#[derive(Debug, Default, Component)]
pub struct ScrollPane {
    pub position: Vec2,
}

#[derive(Debug, Default, Component)]
pub struct TextInput {
    pub active: bool,
}

#[derive(Debug, Component)]
pub struct CursorTimer {
    pub timer: Timer,
    pub was_active: bool,
}

impl Default for CursorTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.5, TimerMode::Repeating),
            was_active: false,
        }
    }
}

#[derive(Debug, Default, Component)]
pub struct FocusableElement {
    pub focused: bool,
}
