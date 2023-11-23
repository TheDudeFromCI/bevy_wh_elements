use bevy::prelude::*;
use bevy::text::BreakLineOn;

use crate::prelude::{
    CursorTimer,
    ElementAlignment,
    ElementDirection,
    NodeText,
    TextInput,
    WhElement,
    WhNode,
};
use crate::{build_node_field, build_text_field, CURSOR_HANDLE};

pub struct WhTextInput<ContainerFlags: Bundle, TextFlags: Bundle> {
    pub container_flags: ContainerFlags,
    pub text_flags: TextFlags,
    pub node: WhNode,
    pub text: NodeText,
}

impl<ContainerFlags: Bundle, TextFlags: Bundle> WhTextInput<ContainerFlags, TextFlags> {
    build_node_field!(node);
    build_text_field!(text);

    pub fn new(container_flags: ContainerFlags, text_flags: TextFlags) -> Box<Self> {
        Box::new(Self {
            container_flags,
            text_flags,
            node: WhNode {
                direction: ElementDirection::Row,
                justify: ElementAlignment::Left,
                alignment: ElementAlignment::Center,
                ..default()
            },
            text: NodeText {
                text: "".into(),
                ..default()
            },
        })
    }
}

impl<ContainerFlags: Bundle, TextFlags: Bundle> WhElement
    for WhTextInput<ContainerFlags, TextFlags>
{
    fn build_child(
        mut self: Box<Self>,
        commands: &mut Commands,
        loader: &AssetServer,
        parent: Option<Entity>,
    ) {
        self.text.wrapping = !self.node.no_wrap;

        let mut cmd = self.node.build_entity(commands, loader);
        cmd.insert(self.container_flags);
        let container_id = cmd.id();

        if let Some(parent) = parent {
            cmd.set_parent(parent);
        }

        let text_style = TextStyle {
            font: match self.text.font {
                Some(path) => loader.load(path),
                None => Default::default(),
            },
            font_size: self.text.size,
            color: self.text.color,
        };

        let mut cmd = commands.spawn((
            self.text_flags,
            TextInput { active: true },
            CursorTimer::default(),
            TextBundle {
                style: Style {
                    align_content: AlignContent::Center,
                    align_self: AlignSelf::Center,
                    ..default()
                },
                text: Text {
                    linebreak_behavior: match self.text.wrapping {
                        true => BreakLineOn::WordBoundary,
                        false => BreakLineOn::NoWrap,
                    },
                    alignment: self.text.alignment,
                    sections: vec![
                        TextSection {
                            value: "".to_string(),
                            style: text_style.clone(),
                        },
                        TextSection {
                            value: "}".to_string(),
                            style: TextStyle {
                                font: CURSOR_HANDLE,
                                color: Color::BLACK,
                                ..text_style.clone()
                            },
                        },
                        TextSection {
                            value: "".to_string(),
                            style: text_style.clone(),
                        },
                    ],
                },
                ..default()
            },
        ));
        cmd.set_parent(container_id);
    }
}
