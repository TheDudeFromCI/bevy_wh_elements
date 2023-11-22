use bevy::prelude::*;
use bevy_wh_elements::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WhElementsPlugin)
        .add_systems(Startup, init)
        .run();
}

fn init(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    ui().build(&mut commands, &asset_server);
}

fn ui() -> BoxedElement {
    WhCanvas::new(()).add_child(
        WhScreen::new(())
            .background(NodeBackground::Image("bg.png".into()))
            .add_child(
                WhScrollPane::new((), ())
                    .background(NodeBackground::Bordered {
                        bg: Color::rgba(0.0, 0.0, 0.0, 0.5),
                        border: Color::WHITE,
                        thickness: Val::Px(1.0),
                    })
                    .size(Val::Percent(50.0), Val::Percent(50.0))
                    .scroll_direction(ScrollDirection::Vertical)
                    .direction(ElementDirection::Column, Val::Px(10.0))
                    .add_children(vec![
                        list_elem("Apple"),
                        list_elem("Banana"),
                        list_elem("Cherry"),
                        list_elem("Date"),
                        list_elem("Orange"),
                        list_elem("Pear"),
                        list_elem("Pineapple"),
                        list_elem("Strawberry"),
                        list_elem("Watermelon"),
                        list_elem("Grape"),
                        list_elem("Kiwi"),
                        list_elem("Lemon"),
                        list_elem("Mango"),
                        list_elem("Peach"),
                        list_elem("Plum"),
                        list_elem("Raspberry"),
                        list_elem("Tomato"),
                        list_elem("Avocado"),
                        list_elem("Blackberry"),
                        list_elem("Blueberry"),
                        list_elem("Coconut"),
                        list_elem("Fig"),
                        list_elem("Grapefruit"),
                        list_elem("Guava"),
                        list_elem("Honeydew"),
                        list_elem("Kumquat"),
                    ]),
            ),
    )
}

fn list_elem(text: &str) -> BoxedElement {
    WhText::new((), ())
        .background(NodeBackground::Color(Color::rgba(0.0, 0.0, 0.0, 0.5)))
        .size(Val::Percent(100.0), Val::Px(20.0))
        .text(text)
}
