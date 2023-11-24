use std::sync::Arc;

use bevy::prelude::*;

use crate::prelude::{BoxedElement, NodeInteraction, OnClickCommandActions, WhElement, WhNode};
use crate::{build_children_field, build_node_field};

pub struct WhButton<Flags: Bundle> {
    pub flags: Flags,
    pub node: WhNode,
    pub children: Vec<BoxedElement>,
    pub on_click: OnClickCommandActions,
}

impl WhButton<()> {
    pub fn new() -> Box<Self> {
        Self::with_flags(())
    }
}

impl<Flags: Bundle> WhButton<Flags> {
    build_node_field!(node);
    build_children_field!(children);

    pub fn with_flags(flags: Flags) -> Box<Self> {
        Box::new(Self {
            flags,
            node: WhNode {
                interaction: NodeInteraction::Button,
                ..default()
            },
            children: Default::default(),
            on_click: Default::default(),
        })
    }

    pub fn on_click<E: Event + Clone>(mut self: Box<Self>, event: E) -> Box<Self> {
        self.on_click.actions.push(Arc::new(move |world| {
            world.send_event(event.clone());
        }));
        self
    }
}

impl<Flags: Bundle> WhElement for WhButton<Flags> {
    fn build_child(
        self: Box<Self>,
        commands: &mut Commands,
        loader: &AssetServer,
        parent: Option<Entity>,
    ) {
        let mut cmd = self.node.build_entity(commands, loader);
        cmd.insert((self.flags, self.on_click));

        if let Some(parent) = parent {
            cmd.set_parent(parent);
        }

        let entity = cmd.id();
        for child in self.children {
            child.build_child(commands, loader, Some(entity));
        }
    }
}
