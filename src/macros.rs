#[macro_export]
macro_rules! build_node_field {
    ($field:ident) => {
        pub fn bg_color(mut self: Box<Self>, color: bevy::prelude::Color) -> Box<Self> {
            self.$field.bg_color = color;
            self
        }

        pub fn bg_img(mut self: Box<Self>, image: impl Into<String>) -> Box<Self> {
            self.$field.bg_img = Some(image.into());

            if self.$field.bg_color == bevy::prelude::Color::NONE {
                self.$field.bg_color = bevy::prelude::Color::WHITE;
            }

            self
        }

        pub fn border(
            mut self: Box<Self>,
            border: bevy::prelude::Color,
            thickness: bevy::prelude::Val,
        ) -> Box<Self> {
            self.$field.border_color = border;
            self.$field.border_thickness = thickness;

            if let Some(active) = &mut self.$field.active_border {
                active.unfocused_color = border;
                active.unfocused_thickness = thickness;
            }

            self
        }

        pub fn size(
            mut self: Box<Self>,
            width: bevy::prelude::Val,
            height: bevy::prelude::Val,
        ) -> Box<Self> {
            self.$field.width = width;
            self.$field.height = height;
            self
        }

        pub fn direction(
            mut self: Box<Self>,
            direction: $crate::prelude::ElementDirection,
            gap: bevy::prelude::Val,
        ) -> Box<Self> {
            self.$field.direction = direction;
            self.$field.gap = gap;
            self
        }

        pub fn justify(
            mut self: Box<Self>,
            justify: $crate::prelude::ElementAlignment,
        ) -> Box<Self> {
            self.$field.justify = justify;
            self
        }

        pub fn align(
            mut self: Box<Self>,
            alignment: $crate::prelude::ElementAlignment,
        ) -> Box<Self> {
            self.$field.alignment = alignment;
            self
        }

        pub fn padding(mut self: Box<Self>, padding: bevy::prelude::UiRect) -> Box<Self> {
            self.$field.padding = padding;
            self
        }

        pub fn margin(mut self: Box<Self>, margin: bevy::prelude::UiRect) -> Box<Self> {
            self.$field.margin = margin;
            self
        }

        pub fn growing(mut self: Box<Self>) -> Box<Self> {
            self.$field.flex_grow = 1.0;
            self
        }

        pub fn no_wrap(mut self: Box<Self>) -> Box<Self> {
            self.$field.no_wrap = true;
            self
        }

        pub fn aspect_ratio(mut self: Box<Self>, ratio: f32) -> Box<Self> {
            self.$field.aspect_ratio = Some(ratio);
            self
        }

        pub fn interaction(
            mut self: Box<Self>,
            interaction: $crate::prelude::NodeInteraction,
        ) -> Box<Self> {
            self.$field.interaction = interaction;
            self
        }

        pub fn change_border_on_active(
            mut self: Box<Self>,
            focused_color: bevy::prelude::Color,
            focused_thickness: bevy::prelude::Val,
        ) -> Box<Self> {
            self.$field.active_border = Some($crate::prelude::BorderChangeOnActive {
                focused_color,
                focused_thickness,
                unfocused_color: self.$field.border_color,
                unfocused_thickness: self.$field.border_thickness,
            });
            self
        }

        pub fn absolute_position(mut self: Box<Self>) -> Box<Self> {
            self.$field.position_type = bevy::prelude::PositionType::Absolute;
            self
        }
    };
}

#[macro_export]
macro_rules! build_children_field {
    ($field:ident) => {
        pub fn add_children(
            mut self: Box<Self>,
            mut children: Vec<$crate::prelude::BoxedElement>,
        ) -> Box<Self> {
            self.$field.append(&mut children);
            self
        }

        pub fn add_child(mut self: Box<Self>, child: $crate::prelude::BoxedElement) -> Box<Self> {
            self.$field.push(child);
            self
        }
    };
}

#[macro_export]
macro_rules! build_text_field {
    ($field:ident) => {
        pub fn text(mut self: Box<Self>, text: impl Into<String>) -> Box<Self> {
            self.$field.text = text.into();
            self
        }

        pub fn font(mut self: Box<Self>, font: impl Into<String>) -> Box<Self> {
            self.$field.font = Some(font.into());
            self
        }

        pub fn font_size(mut self: Box<Self>, size: f32) -> Box<Self> {
            self.$field.size = size;
            self
        }

        pub fn text_color(mut self: Box<Self>, color: bevy::prelude::Color) -> Box<Self> {
            self.$field.color = color;
            self
        }
    };
}
