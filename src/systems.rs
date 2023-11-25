use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use crate::prelude::{
    BorderChangeOnActive,
    CursorTimer,
    FocusableElement,
    OnClickCommandActions,
    RadioButtonElement,
    ScrollPane,
    SetScreenInGroup,
    SetScreenVisible,
    TextInput,
    ToggleScreen,
};

pub(super) fn mouse_scroll_pane(
    query_windows: Query<&Window>,
    query_node: Query<(&Node, &GlobalTransform)>,
    mut query_list: Query<(&mut ScrollPane, &mut Style, &Parent, &Node)>,
    mut mouse_wheel_evs: EventReader<MouseWheel>,
) {
    for ev in mouse_wheel_evs.read() {
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let (parent_node, transform) = query_node.get(parent.get()).unwrap();

            let window = query_windows.get(ev.window).unwrap();
            let Some(cursor_pos) = window.cursor_position() else {
                continue;
            };

            let node_rect = parent_node.logical_rect(transform);
            if !node_rect.contains(cursor_pos) {
                continue;
            }

            let items_size = list_node.size();
            let container_size = parent_node.size();
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
        for (mut text, mut text_input) in query_text_input.iter_mut() {
            if !text_input.active {
                continue;
            }

            match ev.char {
                // Backspace key
                '\u{8}' => {
                    text.sections[0].value.pop();

                    let full_text = text.sections[0].value.clone() + &text.sections[2].value;
                    text_input.set_text(full_text);
                }
                // Delete key
                '\u{7f}' => {
                    text.sections[2].value = text.sections[2].value.chars().skip(1).collect();

                    let full_text = text.sections[0].value.clone() + &text.sections[2].value;
                    text_input.set_text(full_text);
                }
                '\r' => (),
                c => {
                    text.sections[0].value.push(c);

                    let full_text = text.sections[0].value.clone() + &text.sections[2].value;
                    text_input.set_text(full_text);
                }
            };
        }
    }

    for ev in keyboard_evs.read() {
        if ev.state.is_pressed() {
            continue;
        }

        for (mut text, text_input) in query_text_input.iter_mut() {
            if !text_input.active {
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
        debug!("Focused Element: {:?}", focused);
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
        debug!("Unfocused all elements");
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
        text_input.active = focusable.focused;
    }
}

#[allow(clippy::type_complexity)]
pub(super) fn select_radio_button(
    query_clicked: Query<(Entity, &Interaction), (Changed<Interaction>, With<RadioButtonElement>)>,
    mut radio_buttons: Query<(Entity, &mut RadioButtonElement)>,
) {
    let mut selected = None;
    for (entity, interaction) in query_clicked.iter() {
        if let Interaction::Pressed = interaction {
            if let Ok((_, radio_button)) = radio_buttons.get(entity) {
                selected = Some((entity, radio_button.group));
            }
        }
    }

    if let Some((entity, group)) = selected {
        for (other_entity, mut other_radio) in radio_buttons.iter_mut() {
            if other_radio.group == group {
                other_radio.selected = other_entity == entity;
            }
        }
        debug!("Selected Radio Button: {:?} in group: {:?}", entity, group);
    }
}

pub(super) fn change_border_on_focus(
    mut query_focused: Query<
        (
            &mut Style,
            &mut BorderColor,
            &BorderChangeOnActive,
            &FocusableElement,
        ),
        Changed<FocusableElement>,
    >,
) {
    for (mut style, mut border, effects, focus) in query_focused.iter_mut() {
        if focus.focused {
            style.border = UiRect::all(effects.focused_thickness);
            *border = effects.focused_color.into();
        } else {
            style.border = UiRect::all(effects.unfocused_thickness);
            *border = effects.unfocused_color.into();
        }
    }
}

pub(super) fn change_border_on_radio(
    mut query_selected: Query<
        (
            &mut Style,
            &mut BorderColor,
            &BorderChangeOnActive,
            &RadioButtonElement,
        ),
        Changed<RadioButtonElement>,
    >,
) {
    for (mut style, mut border, effects, radio) in query_selected.iter_mut() {
        if radio.selected {
            style.border = UiRect::all(effects.focused_thickness);
            *border = effects.focused_color.into();
        } else {
            style.border = UiRect::all(effects.unfocused_thickness);
            *border = effects.unfocused_color.into();
        }
    }
}

pub(super) fn toggle_screen_listener(
    mut query_screens: Query<(&mut ToggleScreen, &mut Style)>,
    mut set_screen_visible_evs: EventReader<SetScreenVisible>,
) {
    for ev in set_screen_visible_evs.read() {
        for (mut screen, mut style) in query_screens.iter_mut() {
            if screen.screen_id == ev.screen_id {
                screen.active = ev.visible;
                style.display = if ev.visible {
                    Display::Flex
                } else {
                    Display::None
                };
            }
        }
    }
}

pub(super) fn toggle_screen_group_listener(
    mut query_screens: Query<(&mut ToggleScreen, &mut Style)>,
    mut set_screen_in_group_evs: EventReader<SetScreenInGroup>,
) {
    for ev in set_screen_in_group_evs.read() {
        for (mut screen, mut style) in query_screens.iter_mut() {
            if screen.group == Some(ev.group) {
                screen.active = screen.screen_id == ev.screen_id;
                style.display = if screen.active {
                    Display::Flex
                } else {
                    Display::None
                };
            }
        }
    }
}

pub(super) fn on_click_command_actions(
    query_on_click: Query<(&OnClickCommandActions, &Interaction), Changed<Interaction>>,
    mut commands: Commands,
) {
    for (on_click, interaction) in query_on_click.iter() {
        if let Interaction::Pressed = interaction {
            for action in on_click.actions.iter() {
                let a = (*action).clone();
                commands.add(move |world: &mut World| a(world));
            }
        }
    }
}
