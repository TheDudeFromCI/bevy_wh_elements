use bevy::prelude::*;

mod systems;

pub mod components;
pub mod element;
pub mod elements;
pub mod macros;
pub mod node;
pub mod properties;
pub mod text;

pub struct WhElementsPlugin;
impl Plugin for WhElementsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::mouse_scroll_pane);
    }
}

pub mod prelude {
    pub use super::components::*;
    pub use super::element::*;
    pub use super::elements::*;
    pub use super::node::*;
    pub use super::properties::*;
    pub use super::text::*;
    pub use super::WhElementsPlugin;
}
