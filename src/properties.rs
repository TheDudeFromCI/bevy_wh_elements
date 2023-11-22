use bevy::prelude::*;

#[derive(Debug, Default, Clone)]
pub enum NodeBackground {
    #[default]
    None,
    Color(Color),
    Image(String),
    TintedImage {
        image: String,
        tint: Color,
    },
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
    Left,
    Right,
}

#[allow(non_upper_case_globals)]
impl ElementAlignment {
    pub const Top: ElementAlignment = ElementAlignment::Left;
    pub const Bottom: ElementAlignment = ElementAlignment::Right;
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

#[derive(Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ScrollDirection {
    #[default]
    Vertical,
    Horizontal,
    Both,
}
