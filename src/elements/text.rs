use bevy::prelude::*;

use crate::prelude::{ElementAlignment, NodeText, WhElement, WhNode};
use crate::{build_node_field, build_text_field};

pub struct WhText<Flags: Bundle> {
    pub flags: Flags,
    pub node: WhNode,
    pub text: NodeText,
}

impl<Flags: Bundle> WhText<Flags> {
    build_node_field!(node);
    build_text_field!(text);

    pub fn new(flags: Flags) -> Box<Self> {
        Box::new(Self {
            flags,
            node: WhNode::default(),
            text: Default::default(),
        })
    }
}

impl<Flags: Bundle> WhElement for WhText<Flags> {
    fn build_child(
        mut self: Box<Self>,
        commands: &mut Commands,
        loader: &AssetServer,
        parent: Option<Entity>,
    ) {
        self.text.alignment = match self.node.alignment {
            ElementAlignment::Start => TextAlignment::Left,
            ElementAlignment::Center => TextAlignment::Center,
            ElementAlignment::End => TextAlignment::Right,
        };
        self.text.wrapping = !self.node.no_wrap;

        let mut cmd = self.node.build_entity(commands, loader);
        cmd.insert(self.flags);
        self.text.insert_text(&mut cmd, loader);

        if let Some(parent) = parent {
            cmd.set_parent(parent);
        }
    }
}
