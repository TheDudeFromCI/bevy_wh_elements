use std::sync::Arc;

use bevy::prelude::*;

use crate::prelude::{RadioButtonGroup, ScreenGroup, ScreenID};

#[derive(Debug, Default, Component)]
pub struct ScrollPane {
    pub position: Vec2,
}

#[derive(Debug, Default, Component)]
pub struct TextInput {
    pub active: bool,
    cur_text: String,
    clear: bool,
}

impl TextInput {
    pub fn current_text(&self) -> &str {
        &self.cur_text
    }

    pub fn clear(&mut self) {
        self.clear = true;
    }

    pub fn should_clear(&self) -> bool {
        self.clear
    }

    pub(crate) fn set_text(&mut self, text: String) {
        self.cur_text = text;
    }

    pub(crate) fn set_cleared(&mut self) {
        self.clear = false;
        self.cur_text = String::new();
    }
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

#[derive(Debug, Component)]
pub struct RadioButtonElement {
    pub group: RadioButtonGroup,
    pub selected: bool,
}

#[derive(Debug, Default, Component, Clone)]
pub struct BorderChangeOnActive {
    pub focused_color: Color,
    pub unfocused_color: Color,
    pub focused_thickness: Val,
    pub unfocused_thickness: Val,
}

#[derive(Debug, Default, Component)]
pub struct ToggleScreen {
    pub active: bool,
    pub screen_id: ScreenID,
    pub group: Option<ScreenGroup>,
}

#[derive(Default, Component)]
pub struct OnClickCommandActions {
    pub actions: Vec<Arc<CommandAction>>,
}
type CommandAction = dyn Fn(&mut World) + Send + Sync + 'static;
