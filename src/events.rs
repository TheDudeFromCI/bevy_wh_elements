use bevy::prelude::*;

use crate::prelude::{ScreenGroup, ScreenID};

#[derive(Debug, Event, Clone)]
pub struct SetScreenVisible {
    pub screen_id: ScreenID,
    pub visible: bool,
}

impl SetScreenVisible {
    pub fn show(id: ScreenID) -> Self {
        Self {
            screen_id: id,
            visible: true,
        }
    }

    pub fn hide(id: ScreenID) -> Self {
        Self {
            screen_id: id,
            visible: false,
        }
    }
}

#[derive(Debug, Event, Clone)]
pub struct SetScreenInGroup {
    pub group: ScreenGroup,
    pub screen_id: ScreenID,
}

impl SetScreenInGroup {
    pub fn new(group: ScreenGroup, screen_id: ScreenID) -> Self {
        Self { group, screen_id }
    }
}
