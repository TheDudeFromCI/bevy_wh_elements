use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use super::properties::*;
use crate::prelude::{BorderChangeOnActive, FocusableElement, RadioButtonElement};

#[derive(Debug, Clone)]
pub struct WhNode {
    pub bg_color: Color,
    pub bg_img: Option<String>,
    pub border_color: Color,
    pub border_thickness: Val,
    pub width: Val,
    pub height: Val,
    pub direction: ElementDirection,
    pub gap: Val,
    pub justify: ElementAlignment,
    pub alignment: ElementAlignment,
    pub padding: UiRect,
    pub margin: UiRect,
    pub flex_grow: f32,
    pub no_wrap: bool,
    pub aspect_ratio: Option<f32>,
    pub interaction: NodeInteraction,
    pub active_border: Option<BorderChangeOnActive>,
}

impl Default for WhNode {
    fn default() -> Self {
        Self {
            bg_color: Color::NONE,
            bg_img: Default::default(),
            border_color: Color::NONE,
            border_thickness: Val::ZERO,
            width: Default::default(),
            height: Default::default(),
            direction: Default::default(),
            gap: Default::default(),
            justify: Default::default(),
            alignment: Default::default(),
            padding: Default::default(),
            margin: Default::default(),
            flex_grow: Default::default(),
            no_wrap: Default::default(),
            aspect_ratio: Default::default(),
            interaction: Default::default(),
            active_border: Default::default(),
        }
    }
}

impl WhNode {
    pub fn build_entity<'w, 's, 'a>(
        self,
        commands: &'a mut Commands<'w, 's>,
        loader: &AssetServer,
    ) -> EntityCommands<'w, 's, 'a> {
        let style = self.build_style();
        let interaction_clone = self.clone();

        let focus_policy = match self.interaction {
            NodeInteraction::None => FocusPolicy::Pass,
            _ => FocusPolicy::Block,
        };

        let mut cmd = match self.bg_img {
            None => commands.spawn(NodeBundle {
                style,
                focus_policy,
                background_color: self.bg_color.into(),
                border_color: self.border_color.into(),
                ..default()
            }),
            Some(bg_path) => commands.spawn(ImageBundle {
                style,
                focus_policy,
                background_color: self.bg_color.into(),
                image: loader.load(bg_path).into(),
                ..default()
            }),
        };
        interaction_clone.insert_interaction(&mut cmd);

        cmd
    }

    pub fn insert_interaction(self, cmd: &mut EntityCommands) {
        match self.interaction {
            NodeInteraction::None => {}
            NodeInteraction::Button => {
                cmd.insert((Button, Interaction::default()));
            }
            NodeInteraction::Checkbox => {
                todo!();
            }
            NodeInteraction::Radio(group) => {
                cmd.insert((
                    Button,
                    Interaction::default(),
                    RadioButtonElement {
                        group,
                        selected: false,
                    },
                ));

                if let Some(active) = self.active_border {
                    cmd.insert(active);
                }
            }
            NodeInteraction::Focusable => {
                cmd.insert((Button, Interaction::default(), FocusableElement::default()));
            }
        };
    }

    pub fn build_style(&self) -> Style {
        let flex_direction = match self.direction {
            ElementDirection::Row => FlexDirection::Row,
            ElementDirection::Column => FlexDirection::Column,
        };

        let justify_content = match self.justify {
            ElementAlignment::Left => JustifyContent::FlexStart,
            ElementAlignment::Center => JustifyContent::Center,
            ElementAlignment::Right => JustifyContent::FlexEnd,
        };

        let align_content = match self.alignment {
            ElementAlignment::Left => AlignContent::FlexStart,
            ElementAlignment::Center => AlignContent::Center,
            ElementAlignment::Right => AlignContent::FlexEnd,
        };

        let flex_wrap = match self.no_wrap {
            true => FlexWrap::NoWrap,
            false => FlexWrap::Wrap,
        };

        Style {
            flex_direction,
            flex_wrap,
            justify_content,
            align_content,
            flex_grow: self.flex_grow,
            row_gap: self.gap,
            column_gap: self.gap,
            width: self.width,
            height: self.height,
            padding: self.padding,
            margin: self.margin,
            aspect_ratio: self.aspect_ratio,
            border: UiRect::all(self.border_thickness),
            ..default()
        }
    }
}
