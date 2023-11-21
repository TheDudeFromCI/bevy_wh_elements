use bevy::prelude::*;

pub trait WhElement {
    fn build_child(
        self: Box<Self>,
        commands: &mut Commands,
        loader: &AssetServer,
        parent: Option<Entity>,
    );

    fn build(self: Box<Self>, commands: &mut Commands, loader: &AssetServer) {
        self.build_child(commands, loader, None);
    }
}

pub trait WhFlags: Bundle {}
impl<T> WhFlags for T where T: Bundle {}

pub type BoxedElement = Box<dyn WhElement + Send + Sync>;
