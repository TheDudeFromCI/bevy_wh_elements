use bevy::prelude::*;

use crate::prelude::{AssetReference, ElementDirection, NodeText, WhElement, WhNode};
use crate::{build_node_field, build_text_field};

pub struct WhText<ContainerFlags: Bundle, TextFlags: Bundle> {
    pub container_flags: ContainerFlags,
    pub text_flags: TextFlags,
    pub node: WhNode,
    pub text: NodeText,
}

impl WhText<(), ()> {
    pub fn new() -> Box<Self> {
        Self::with_flags((), ())
    }
}

impl<ContainerFlags: Bundle, TextFlags: Bundle> WhText<ContainerFlags, TextFlags> {
    build_node_field!(node);
    build_text_field!(text);

    pub fn with_flags(container_flags: ContainerFlags, text_flags: TextFlags) -> Box<Self> {
        Box::new(Self {
            container_flags,
            text_flags,
            node: WhNode {
                direction: ElementDirection::Row,
                ..default()
            },
            text: Default::default(),
        })
    }
}

impl<ContainerFlags: Bundle, TextFlags: Bundle> WhElement for WhText<ContainerFlags, TextFlags> {
    fn build_child(
        mut self: Box<Self>,
        commands: &mut Commands,
        loader: &mut AssetReference,
        parent: Option<Entity>,
    ) {
        self.text.wrapping = !self.node.no_wrap;

        let mut cmd = self.node.build_entity(commands, loader);
        cmd.insert(self.container_flags);
        let container_id = cmd.id();

        if let Some(parent) = parent {
            cmd.set_parent(parent);
        }

        let mut cmd = self.text.build_entity(commands, loader);
        cmd.insert(self.text_flags);
        cmd.set_parent(container_id);
    }
}
