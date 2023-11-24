use bevy::asset::load_internal_binary_asset;
use bevy::prelude::*;
use events::{SetScreenInGroup, SetScreenVisible};

mod systems;

pub mod components;
pub mod element;
pub mod elements;
pub mod events;
pub mod macros;
pub mod node;
pub mod properties;
pub mod text;

pub(crate) const CURSOR_HANDLE: Handle<Font> = Handle::weak_from_u128(10482756907980398621);

pub struct WhElementsPlugin;
impl Plugin for WhElementsPlugin {
    fn build(&self, app_: &mut App) {
        // Credits to `bevy_simple_text_input` for providing this custom font.
        // <https://github.com/rparrett/bevy_simple_text_input>
        load_internal_binary_asset!(
            app_,
            CURSOR_HANDLE,
            "../assets/fonts/cursor.ttf",
            |bytes: &[u8], _path: String| { Font::try_from_bytes(bytes.to_vec()).unwrap() }
        );

        app_.add_event::<SetScreenVisible>()
            .add_event::<SetScreenInGroup>()
            .add_systems(
                Update,
                (
                    systems::focus_on_element.in_set(SystemSets::UpdateFocus),
                    systems::unfocus_elements.in_set(SystemSets::UpdateFocus),
                    systems::text_input_from_focus.in_set(SystemSets::InheritFocus),
                    systems::mouse_scroll_pane.in_set(SystemSets::UserInteraction),
                    systems::keyboard_text_input.in_set(SystemSets::UserInteraction),
                    systems::select_radio_button.in_set(SystemSets::UserInteraction),
                    systems::text_cursor_blinker.in_set(SystemSets::Animations),
                    systems::change_border_on_focus.in_set(SystemSets::Animations),
                    systems::change_border_on_radio.in_set(SystemSets::Animations),
                    systems::toggle_screen_listener.in_set(SystemSets::Animations),
                    systems::toggle_screen_group_listener.in_set(SystemSets::Animations),
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
            )
            .configure_sets(
                Update,
                (
                    SystemSets::Animations.ambiguous_with(SystemSets::UpdateFocus),
                    SystemSets::Animations.ambiguous_with(SystemSets::InheritFocus),
                    SystemSets::Animations.ambiguous_with(SystemSets::UserInteraction),
                    SystemSets::Animations.ambiguous_with(SystemSets::Animations),
                ),
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
    pub use super::events::*;
    pub use super::node::*;
    pub use super::properties::*;
    pub use super::text::*;
    pub use super::WhElementsPlugin;
}
