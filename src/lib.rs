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
            (
                systems::mouse_scroll_pane.in_set(SystemSets::UserInteraction),
                systems::keyboard_text_input.in_set(SystemSets::UserInteraction),
                systems::text_cursor_blinker.in_set(SystemSets::Animations),
                systems::focus_on_element.in_set(SystemSets::UpdateFocus),
                systems::unfocus_elements.in_set(SystemSets::UpdateFocus),
                systems::text_input_from_focus.in_set(SystemSets::InheritFocus),
            ),
        )
        .configure_sets(
            Update,
            (
                SystemSets::UpdateFocus,
                SystemSets::InheritFocus,
                SystemSets::UserInteraction,
            )
                .chain(),
        );
    }
}

#[derive(Debug, SystemSet, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemSets {
    UserInteraction,
    UpdateFocus,
    InheritFocus,
    Animations,
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
