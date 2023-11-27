use bevy::prelude::*;

use crate::prelude::AssetReference;

pub trait WhElement {
    fn build_child(
        self: Box<Self>,
        commands: &mut Commands,
        loader: &mut AssetReference,
        parent: Option<Entity>,
    );

    fn build(self: Box<Self>, commands: &mut Commands, loader: &mut AssetReference) {
        self.build_child(commands, loader, None);
    }
}

pub type BoxedElement = Box<dyn WhElement + Send + Sync>;
