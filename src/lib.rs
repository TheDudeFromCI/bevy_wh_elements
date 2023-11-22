use bevy::prelude::*;

mod systems;

pub mod element;
pub mod elements;
pub mod macros;
pub mod node;
pub mod properties;
pub mod text;

pub struct WhElementsPlugin;
impl Plugin for WhElementsPlugin {
    fn build(&self, _app: &mut App) {}
}

pub mod prelude {
    pub use super::element::*;
    pub use super::elements::*;
    pub use super::node::*;
    pub use super::properties::*;
    pub use super::text::*;
    pub use super::WhElementsPlugin;
}
