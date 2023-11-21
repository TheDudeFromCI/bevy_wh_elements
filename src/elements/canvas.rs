use bevy::prelude::*;

use crate::build_children_field;
use crate::prelude::{BoxedElement, WhElement};

pub struct WhCanvas<Flags: Bundle> {
    pub flags: Flags,
    pub children: Vec<BoxedElement>,
}

impl<Flags: Bundle> WhCanvas<Flags> {
    build_children_field!();

    pub fn new(flags: Flags) -> Box<Self> {
        Box::new(Self {
            flags,
            children: Default::default(),
        })
    }
}

impl<Flags: Bundle> WhElement for WhCanvas<Flags> {
    fn build_child(
        self: Box<Self>,
        commands: &mut Commands,
        loader: &AssetServer,
        parent: Option<Entity>,
    ) {
        let mut cmd = commands.spawn((
            self.flags,
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::NONE.into(),
                ..default()
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
