use bevy::prelude::*;

use crate::prelude::{ScreenGroup, ScreenID};

#[derive(Debug, Event)]
pub struct SetScreenVisible {
    pub screen_id: ScreenID,
    pub visible: bool,
}

#[derive(Debug, Event)]
pub struct SetScreenInGroup {
    pub group: ScreenGroup,
    pub screen_id: ScreenID,
}
