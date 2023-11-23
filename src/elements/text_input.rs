use bevy::prelude::*;
use bevy::text::BreakLineOn;

use crate::prelude::{
    CursorTimer,
    ElementAlignment,
    ElementDirection,
    NodeInteraction,
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

impl WhTextInput<(), ()> {
    pub fn new() -> Box<Self> {
        Self::with_flags((), ())
    }
}

impl<ContainerFlags: Bundle, TextFlags: Bundle> WhTextInput<ContainerFlags, TextFlags> {
    build_node_field!(node);
    build_text_field!(text);

    pub fn with_flags(container_flags: ContainerFlags, text_flags: TextFlags) -> Box<Self> {
        Box::new(Self {
            container_flags,
            text_flags,
            node: WhNode {
                direction: ElementDirection::Row,
                justify: ElementAlignment::Left,
                alignment: ElementAlignment::Center,
                padding: UiRect::all(Val::Px(2.0)),
                interaction: NodeInteraction::Focusable,
                ..default()
            },
            text: NodeText {
                text: "".into(),
                alignment: TextAlignment::Left,
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
        let text_vert_align = match &self.node.alignment {
            ElementAlignment::Left => AlignSelf::FlexStart,
            ElementAlignment::Center => AlignSelf::Center,
            ElementAlignment::Right => AlignSelf::FlexEnd,
        };

        let mut cmd = self.node.build_entity(commands, loader);
        cmd.insert(self.container_flags);
        let container_id = cmd.id();

        if let Some(parent) = parent {
            cmd.set_parent(parent);
        }

        let mut cmd = commands.spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                overflow: Overflow::clip(),
                align_self: text_vert_align,
                justify_content: JustifyContent::FlexEnd,
                align_content: AlignContent::Center,
                flex_direction: FlexDirection::Row,
                max_width: Val::Percent(100.0),
                ..default()
            },
            ..default()
        });
        cmd.set_parent(container_id);
        let overflow_id = cmd.id();

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
            TextInput::default(),
            CursorTimer::default(),
            TextBundle {
                style: Style {
                    align_content: AlignContent::Center,
                    align_self: AlignSelf::Center,
                    ..default()
                },
                text: Text {
                    linebreak_behavior: match self.text.wrapping {
                        // TODO: This is a hack to get around the fact that
                        //       bevy doesn't support wrapping on long words.
                        // Waiting on: <https://github.com/bevyengine/bevy/issues/10710>
                        true => BreakLineOn::AnyCharacter,
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
                                color: Color::NONE,
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
        cmd.set_parent(overflow_id);
    }
}
