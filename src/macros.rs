#[macro_export]
macro_rules! build_node_field {
    () => {
        pub fn background(
            mut self: Box<Self>,
            background: $crate::prelude::NodeBackground,
        ) -> Box<Self> {
            self.node.background = background;
            self
        }

        pub fn size(
            mut self: Box<Self>,
            width: bevy::prelude::Val,
            height: bevy::prelude::Val,
        ) -> Box<Self> {
            self.node.width = width;
            self.node.height = height;
            self
        }

        pub fn direction(
            mut self: Box<Self>,
            direction: $crate::prelude::ElementDirection,
            gap: bevy::prelude::Val,
        ) -> Box<Self> {
            self.node.direction = direction;
            self.node.gap = gap;
            self
        }

        pub fn justify(mut self: Box<Self>, justify: $crate::prelude::ElementJustify) -> Box<Self> {
            self.node.justify = justify;
            self
        }

        pub fn padding(mut self: Box<Self>, padding: bevy::prelude::UiRect) -> Box<Self> {
            self.node.padding = padding;
            self
        }

        pub fn margin(mut self: Box<Self>, margin: bevy::prelude::UiRect) -> Box<Self> {
            self.node.margin = margin;
            self
        }

        pub fn growing(mut self: Box<Self>) -> Box<Self> {
            self.node.flex_grow = 1.0;
            self
        }

        pub fn wrap_contents(mut self: Box<Self>) -> Box<Self> {
            self.node.flex_wrap = true;
            self
        }

        pub fn aspect_ratio(mut self: Box<Self>, ratio: f32) -> Box<Self> {
            self.node.aspect_ratio = Some(ratio);
            self
        }

        pub fn interaction(
            mut self: Box<Self>,
            interaction: $crate::prelude::NodeInteraction,
        ) -> Box<Self> {
            self.node.interaction = interaction;
            self
        }
    };
}

#[macro_export]
macro_rules! build_children_field {
    () => {
        pub fn add_children(
            mut self: Box<Self>,
            mut children: Vec<$crate::prelude::BoxedElement>,
        ) -> Box<Self> {
            self.children.append(&mut children);
            self
        }

        pub fn add_child(mut self: Box<Self>, child: $crate::prelude::BoxedElement) -> Box<Self> {
            self.children.push(child);
            self
        }
    };
}
