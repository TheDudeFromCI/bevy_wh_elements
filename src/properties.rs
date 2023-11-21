use bevy::prelude::*;
use bevy::text::BreakLineOn;

#[derive(Debug, Default, Clone)]
pub enum NodeBackground {
    #[default]
    None,
    Color(Color),
    Image(String),
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementDirection {
    #[default]
    Column,
    Row,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementJustify {
    #[default]
    Center,
    Start,
    End,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ElementAlignment {
    #[default]
    Center,
    Start,
    End,
}

#[derive(Debug, Clone)]
pub struct NodeText {
    pub font: Option<String>,
    pub size: f32,
    pub color: Color,
    pub alignment: ElementAlignment,
    pub justify: ElementJustify,
    pub text: String,
}

impl Default for NodeText {
    fn default() -> Self {
        Self {
            font: Default::default(),
            size: 20.0,
            color: Color::BLACK,
            alignment: ElementAlignment::Center,
            justify: ElementJustify::Start,
            text: "Text".into(),
        }
    }
}

impl NodeText {
    pub fn into_text_bundle(self, loader: &AssetServer) -> TextBundle {
        let alignment = match self.alignment {
            ElementAlignment::Center => AlignContent::Center,
            ElementAlignment::Start => AlignContent::FlexStart,
            ElementAlignment::End => AlignContent::FlexEnd,
        };

        let justify = match self.justify {
            ElementJustify::Center => TextAlignment::Center,
            ElementJustify::Start => TextAlignment::Left,
            ElementJustify::End => TextAlignment::Right,
        };

        let font = match self.font {
            Some(path) => loader.load(path),
            None => Default::default(),
        };

        TextBundle {
            style: Style {
                align_content: alignment,
                ..default()
            },
            text: Text {
                linebreak_behavior: BreakLineOn::WordBoundary,
                alignment: justify,
                sections: vec![TextSection {
                    value: self.text,
                    style: TextStyle {
                        font,
                        font_size: self.size,
                        color: self.color,
                    },
                }],
            },
            ..default()
        }
    }
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
