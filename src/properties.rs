use bevy::prelude::*;

#[derive(Debug, Default, Clone)]
pub enum NodeBackground {
    #[default]
    None,
    Color(Color),
    Image(String),
    Bordered {
        bg: Color,
        border: Color,
        thickness: Val,
    },
}

#[derive(Debug, Default, Clone)]
pub enum NodeInteraction {
    #[default]
    None,
    Radio(RadioButtonGroup),
    Button,
    Checkbox,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RadioButtonGroup(pub u64);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementDirection {
    #[default]
    Column,
    Row,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementAlignment {
    #[default]
    Center,
    Start,
    End,
}

#[derive(Debug, Default, Clone)]
pub enum NodeEffect {
    #[default]
    None,
    BackgroundTint {
        active: Color,
        inactive: Color,
    },
}

#[derive(Debug, Default, Component)]
pub struct SelectableNode {
    pub effect: NodeEffect,
}

#[derive(Debug, Event)]
pub struct SelectNodeEvent {
    pub entity: Entity,
    pub selected: bool,
}
