use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;

mod systems;

pub mod components;
pub mod element;
pub mod elements;
pub mod macros;
pub mod node;
pub mod properties;
pub mod text;

pub(crate) const CURSOR_HANDLE: Handle<Font> = Handle::weak_from_u128(10482756907980398621);

pub struct WhElementsPlugin;
impl Plugin for WhElementsPlugin {
    fn build(&self, app: &mut App) {
        // Credits to `bevy_simple_text_input` for providing this custom font.
        // <https://github.com/rparrett/bevy_simple_text_input>
        load_internal_binary_asset!(
            app,
            CURSOR_HANDLE,
            "../assets/fonts/cursor.ttf",
            |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
        );

        app.add_systems(
            Update,
            (systems::mouse_scroll_pane, systems::keyboard_text_input),
        );
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
