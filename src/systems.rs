use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use crate::prelude::{ScrollPane, TextInput};

pub(super) fn mouse_scroll_pane(
    query_node: Query<&Node>,
    mut query_list: Query<(&mut ScrollPane, &mut Style, &Parent, &Node, &Interaction)>,
    mut mouse_wheel_evs: EventReader<MouseWheel>,
) {
    for ev in mouse_wheel_evs.read() {
        for (mut scrolling_list, mut style, parent, list_node, interaction) in &mut query_list {
            if interaction != &Interaction::Hovered {
                continue;
            }

            let items_size = list_node.size();
            let container_size = query_node.get(parent.get()).unwrap().size();
            let max_scroll = (items_size - container_size).max(Vec2::ZERO);

            let scroll_amount = Vec2::new(ev.x, ev.y);
            let delta = match ev.unit {
                MouseScrollUnit::Line => scroll_amount * 20.,
                MouseScrollUnit::Pixel => scroll_amount,
            };

            scrolling_list.position += delta;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, Vec2::ZERO);

            style.left = Val::Px(scrolling_list.position.x);
            style.top = Val::Px(scrolling_list.position.y);
        }
    }
}

pub(super) fn keyboard_text_input(
    mut keyboard_evs: EventReader<KeyboardInput>,
    mut character_evs: EventReader<ReceivedCharacter>,
    mut query_text_input: Query<(&mut Text, &mut TextInput)>,
) {
    if keyboard_evs.is_empty() && character_evs.is_empty() {
        return;
    }

    for ev in character_evs.read() {
        for (mut text, text_input) in query_text_input.iter_mut() {
            if !text_input.active {
                continue;
            }

            match ev.char {
                // Backspace key
                '\u{8}' => {
                    text.sections[0].value.pop();
                }
                // Delete key
                '\u{7f}' => {
                    text.sections[2].value = text.sections[2].value.chars().skip(1).collect();
                }
                '\r' => (),
                c => {
                    text.sections[0].value.push(c);
                }
            };
        }
    }

    for ev in keyboard_evs.read() {
        if ev.state.is_pressed() {
            continue;
        }

        for (mut text, mut text_input) in query_text_input.iter_mut() {
            if !text_input.active {
                continue;
            }

            match ev.key_code {
                Some(KeyCode::Escape) => {
                    text_input.active = false;
                }
                Some(KeyCode::Left) => {
                    if let Some(behind) = text.sections[0].value.pop() {
                        text.sections[2].value.insert(0, behind);
                    }
                }
                Some(KeyCode::Right) => {
                    if !text.sections[2].value.is_empty() {
                        let ahead = text.sections[2].value.remove(0);
                        text.sections[0].value.push(ahead);
                    }
                }
                _ => {}
            }

            if text.sections[2].value.is_empty() {
                text.sections[1].value = "}".to_string();
            } else {
                text.sections[1].value = "|".to_string();
            }
        }
    }
}
