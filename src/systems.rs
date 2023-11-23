use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use crate::prelude::{CursorTimer, FocusableElement, ScrollPane, TextInput};

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
    mut query_text_input: Query<(&mut Text, &TextInput)>,
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

        for (mut text, focus) in query_text_input.iter_mut() {
            if !focus.active {
                continue;
            }

            match ev.key_code {
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

pub(super) fn text_cursor_blinker(
    time: Res<Time>,
    mut query_text_input: Query<(&mut CursorTimer, &mut Text, &TextInput)>,
) {
    for (mut timer, mut text, text_input) in query_text_input.iter_mut() {
        if text_input.active != timer.was_active {
            timer.timer.reset();
            timer.was_active = text_input.active;
        } else if !timer.timer.tick(time.delta()).just_finished() {
            continue;
        }

        let cursor_color = text.sections[1].style.color;
        let text_color = text.sections[0].style.color;

        if !text_input.active || cursor_color != Color::NONE {
            text.sections[1].style.color = Color::NONE;
        } else {
            text.sections[1].style.color = text_color;
        }
    }
}

#[allow(clippy::type_complexity)]
pub(super) fn focus_on_element(
    query_pressed_focus: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<FocusableElement>),
    >,
    mut query_focusable: Query<(Entity, &mut FocusableElement)>,
) {
    let mut focused = None;
    for (entity, interaction) in query_pressed_focus.iter() {
        if let Interaction::Pressed = interaction {
            focused = Some(entity);
        }
    }

    if let Some(focused) = focused {
        for (entity, mut focusable) in query_focusable.iter_mut() {
            focusable.focused = entity == focused;
        }
        info!("focused: {:?}", focused);
    }
}

pub(super) fn unfocus_elements(
    mut keyboard_evs: EventReader<KeyboardInput>,
    mut query_focusable: Query<&mut FocusableElement>,
) {
    if keyboard_evs
        .read()
        .any(|ev| ev.key_code == Some(KeyCode::Escape))
    {
        for mut focusable in query_focusable.iter_mut() {
            focusable.focused = false;
        }
        info!("unfocused all");
    }
}

pub(super) fn text_input_from_focus(
    query_hierarchy: Query<&Parent>,
    query_focusable: Query<&FocusableElement>,
    mut query_text_input: Query<(&mut TextInput, &Parent)>,
) {
    for (mut text_input, parent) in query_text_input.iter_mut() {
        let overflow_id = parent.get();
        let container_id = query_hierarchy.get(overflow_id).unwrap().get();
        let focusable = query_focusable.get(container_id).unwrap();

        if text_input.active != focusable.focused {
            info!("text_input.active: {:?}", focusable.focused);
        }

        text_input.active = focusable.focused;
    }
}
