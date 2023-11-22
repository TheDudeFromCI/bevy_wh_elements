use bevy::prelude::*;
use bevy::ui::FocusPolicy;

use crate::prelude::{
    BoxedElement,
    ElementAlignment,
    ElementDirection,
    NodeBackground,
    NodeInteraction,
    ScrollDirection,
    ScrollPane,
    WhElement,
    WhNode,
};
use crate::{build_children_field, build_node_field};

pub struct WhScrollPane<ContainerFlags: Bundle, PanelFlags: Bundle> {
    pub container_flags: ContainerFlags,
    pub panel_flags: PanelFlags,
    pub node: WhNode,
    pub children: Vec<BoxedElement>,
    pub scroll_direction: ScrollDirection,
}

impl<ContainerFlags: Bundle, PanelFlags: Bundle> WhScrollPane<ContainerFlags, PanelFlags> {
    build_node_field!(node);
    build_children_field!(children);

    pub fn new(container_flags: ContainerFlags, panel_flags: PanelFlags) -> Box<Self> {
        Box::new(Self {
            container_flags,
            panel_flags,
            node: WhNode {
                no_wrap: true,
                ..default()
            },
            children: Default::default(),
            scroll_direction: Default::default(),
        })
    }

    pub fn scroll_direction(mut self: Box<Self>, scroll_direction: ScrollDirection) -> Box<Self> {
        self.scroll_direction = scroll_direction;
        self
    }
}

impl<ContainerFlags: Bundle, PanelFlags: Bundle> WhElement
    for WhScrollPane<ContainerFlags, PanelFlags>
{
    fn build_child(
        self: Box<Self>,
        commands: &mut Commands,
        loader: &AssetServer,
        parent: Option<Entity>,
    ) {
        let container_style = Style {
            flex_direction: match self.scroll_direction {
                ScrollDirection::Vertical => FlexDirection::Column,
                ScrollDirection::Horizontal => FlexDirection::Row,
                ScrollDirection::Both => FlexDirection::Column,
            },
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::Center,
            flex_grow: self.node.flex_grow,
            width: self.node.width,
            height: self.node.height,
            margin: self.node.margin,
            aspect_ratio: self.node.aspect_ratio,
            overflow: Overflow::clip(),
            ..default()
        };

        let panel_style = Style {
            flex_direction: match self.node.direction {
                ElementDirection::Row => FlexDirection::Row,
                ElementDirection::Column => FlexDirection::Column,
            },
            justify_content: match self.node.justify {
                ElementAlignment::Left => JustifyContent::FlexStart,
                ElementAlignment::Center => JustifyContent::Center,
                ElementAlignment::Right => JustifyContent::FlexEnd,
            },
            align_content: match self.node.alignment {
                ElementAlignment::Left => AlignContent::FlexStart,
                ElementAlignment::Center => AlignContent::Center,
                ElementAlignment::Right => AlignContent::FlexEnd,
            },
            flex_wrap: match self.node.no_wrap {
                true => FlexWrap::NoWrap,
                false => FlexWrap::Wrap,
            },
            row_gap: self.node.gap,
            column_gap: self.node.gap,
            padding: self.node.padding,
            width: match self.scroll_direction {
                ScrollDirection::Vertical => Val::Percent(100.0),
                ScrollDirection::Horizontal => Val::Auto,
                ScrollDirection::Both => Val::Auto,
            },
            height: match self.scroll_direction {
                ScrollDirection::Vertical => Val::Auto,
                ScrollDirection::Horizontal => Val::Percent(100.0),
                ScrollDirection::Both => Val::Auto,
            },
            align_self: AlignSelf::Stretch,
            ..default()
        };

        let mut cmd = match self.node.background {
            NodeBackground::None => commands.spawn((
                self.container_flags,
                NodeBundle {
                    style: container_style,
                    background_color: Color::NONE.into(),
                    ..default()
                },
            )),
            NodeBackground::Color(color) => commands.spawn((
                self.container_flags,
                NodeBundle {
                    style: container_style,
                    background_color: color.into(),
                    ..default()
                },
            )),
            NodeBackground::Image(bg_path) => commands.spawn((
                self.container_flags,
                ImageBundle {
                    style: container_style,
                    image: loader.load(bg_path).into(),
                    ..default()
                },
            )),
            NodeBackground::Bordered {
                bg,
                border,
                thickness,
            } => commands.spawn((
                self.container_flags,
                NodeBundle {
                    style: Style {
                        border: UiRect::all(thickness),
                        ..container_style
                    },
                    background_color: bg.into(),
                    border_color: border.into(),
                    ..default()
                },
            )),
        };
        let container_id = cmd.id();

        if let Some(parent) = parent {
            cmd.set_parent(parent);
        }

        let mut cmd = commands.spawn((
            self.panel_flags,
            ScrollPane::default(),
            Interaction::default(),
            NodeBundle {
                style: panel_style,
                focus_policy: match self.node.interaction {
                    NodeInteraction::None => FocusPolicy::Pass,
                    _ => FocusPolicy::Block,
                },
                ..default()
            },
        ));
        match self.node.interaction {
            NodeInteraction::None => {}
            NodeInteraction::Radio(_) => {
                cmd.insert(Button);
            }
            NodeInteraction::Button => {
                cmd.insert(Button);
            }
            NodeInteraction::Checkbox => {
                cmd.insert(Button);
            }
        };

        cmd.set_parent(container_id);
        let panel_id = cmd.id();

        for child in self.children {
            child.build_child(commands, loader, Some(panel_id));
        }
    }
}
