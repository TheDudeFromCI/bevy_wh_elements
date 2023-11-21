use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use super::properties::*;

#[derive(Default)]
pub struct WhNode {
    pub background: NodeBackground,
    pub width: Val,
    pub height: Val,
    pub direction: ElementDirection,
    pub gap: Val,
    pub justify: ElementJustify,
    pub padding: UiRect,
    pub margin: UiRect,
    pub flex_grow: f32,
    pub flex_wrap: bool,
    pub aspect_ratio: Option<f32>,
    pub interaction: NodeInteraction,
}

impl WhNode {
    pub fn build_entity<'w, 's, 'a>(
        self,
        commands: &'a mut Commands<'w, 's>,
        loader: &AssetServer,
    ) -> EntityCommands<'w, 's, 'a> {
        let style = self.build_style();

        let focus_policy = match self.interaction {
            NodeInteraction::None => FocusPolicy::Pass,
            _ => FocusPolicy::Block,
        };

        let mut cmd = match self.background {
            NodeBackground::None => commands.spawn(NodeBundle {
                style,
                focus_policy,
                background_color: Color::NONE.into(),
                ..default()
            }),
            NodeBackground::Color(color) => commands.spawn(NodeBundle {
                style,
                focus_policy,
                background_color: color.into(),
                ..default()
            }),
            NodeBackground::Image(bg_path) => commands.spawn(ImageBundle {
                style,
                focus_policy,
                image: loader.load(bg_path).into(),
                ..default()
            }),
            NodeBackground::Bordered {
                bg,
                border,
                thickness,
            } => commands.spawn(NodeBundle {
                style: Style {
                    border: UiRect::all(thickness),
                    ..style
                },
                focus_policy,
                background_color: bg.into(),
                border_color: border.into(),
                ..default()
            }),
        };

        match self.interaction {
            NodeInteraction::None => {}
            NodeInteraction::Radio(_) => {
                cmd.insert((Button, Interaction::default()));
            }
            NodeInteraction::Button => {
                cmd.insert((Button, Interaction::default()));
            }
            NodeInteraction::Checkbox => {
                cmd.insert((Button, Interaction::default()));
            }
        };

        cmd
    }

    pub fn build_style(&self) -> Style {
        let flex_direction = match self.direction {
            ElementDirection::Row => FlexDirection::Row,
            ElementDirection::Column => FlexDirection::Column,
        };

        let justify_content = match self.justify {
            ElementJustify::Start => JustifyContent::FlexStart,
            ElementJustify::Center => JustifyContent::Center,
            ElementJustify::End => JustifyContent::FlexEnd,
        };

        let flex_wrap = match self.flex_wrap {
            true => FlexWrap::Wrap,
            false => FlexWrap::NoWrap,
        };

        Style {
            flex_direction,
            flex_wrap,
            justify_content,
            align_items: AlignItems::Center,
            flex_grow: self.flex_grow,
            row_gap: self.gap,
            column_gap: self.gap,
            width: self.width,
            height: self.height,
            padding: self.padding,
            margin: self.margin,
            aspect_ratio: self.aspect_ratio,
            ..default()
        }
    }
}
