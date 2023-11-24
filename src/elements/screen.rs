use bevy::prelude::*;

use crate::prelude::{BoxedElement, ScreenGroup, ScreenID, ToggleScreen, WhElement, WhNode};
use crate::{build_children_field, build_node_field};

pub struct WhScreen<Flags: Bundle> {
    pub flags: Flags,
    pub node: WhNode,
    pub children: Vec<BoxedElement>,
    pub id: ScreenID,
    pub group: Option<ScreenGroup>,
}

impl WhScreen<()> {
    pub fn new(id: ScreenID) -> Box<Self> {
        Self::with_flags((), id)
    }
}

impl<Flags: Bundle> WhScreen<Flags> {
    build_node_field!(node);
    build_children_field!(children);

    pub fn with_flags(flags: Flags, id: ScreenID) -> Box<Self> {
        Box::new(Self {
            flags,
            node: WhNode {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::None,
                position_type: PositionType::Absolute,
                ..default()
            },
            children: Default::default(),
            id,
            group: None,
        })
    }

    pub fn shown(mut self: Box<Self>) -> Box<Self> {
        self.node.display = Display::Flex;
        self
    }

    pub fn in_group(mut self: Box<Self>, group: ScreenGroup) -> Box<Self> {
        self.group = Some(group);
        self
    }
}

impl<Flags: Bundle> WhElement for WhScreen<Flags> {
    fn build_child(
        self: Box<Self>,
        commands: &mut Commands,
        loader: &AssetServer,
        parent: Option<Entity>,
    ) {
        let shown = self.node.display == Display::Flex;

        let mut cmd = self.node.build_entity(commands, loader);
        cmd.insert((
            self.flags,
            ToggleScreen {
                active: shown,
                screen_id: self.id,
                group: self.group,
            },
        ));

        if let Some(parent) = parent {
            cmd.set_parent(parent);
        }

        let entity = cmd.id();
        for child in self.children {
            child.build_child(commands, loader, Some(entity));
        }
    }
}
