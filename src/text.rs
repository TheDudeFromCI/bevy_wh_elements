use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::text::{BreakLineOn, TextLayoutInfo};
use bevy::ui::widget::TextFlags;
use bevy::ui::ContentSize;

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
    pub fn insert_text(self, commands: &mut EntityCommands, loader: &AssetServer) {
        commands.insert((
            Text {
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
            TextLayoutInfo::default(),
            TextFlags::default(),
            ContentSize::default(),
        ));
    }
}
