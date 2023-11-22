use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use crate::prelude::ScrollPane;

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
