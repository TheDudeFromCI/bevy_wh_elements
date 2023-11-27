use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::text::BreakLineOn;

use crate::prelude::AssetReference;

#[derive(Debug, Clone)]
pub struct NodeText {
    pub font: Option<String>,
    pub size: f32,
    pub color: Color,
    pub text: String,
    pub alignment: TextAlignment,
    pub wrapping: bool,
}

impl Default for NodeText {
    fn default() -> Self {
        Self {
            font: Default::default(),
            size: 20.0,
            color: Color::BLACK,
            text: "Text".into(),
            alignment: TextAlignment::Left,
            wrapping: true,
        }
    }
}

impl NodeText {
    pub fn build_entity<'w, 's, 'a>(
        self,
        commands: &'a mut Commands<'w, 's>,
        loader: &mut AssetReference,
    ) -> EntityCommands<'w, 's, 'a> {
        commands.spawn(TextBundle {
            style: Style {
                align_content: AlignContent::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            text: Text {
                linebreak_behavior: match self.wrapping {
                    true => BreakLineOn::WordBoundary,
                    false => BreakLineOn::NoWrap,
                },
                alignment: self.alignment,
                sections: vec![TextSection {
                    value: self.text,
                    style: TextStyle {
                        font: match self.font {
                            Some(path) => loader.load(path),
                            None => Default::default(),
                        },
                        font_size: self.size,
                        color: self.color,
                    },
                }],
            },
            ..default()
        })
    }
}
