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
    ui().build(&mut commands, &asset_server);
}

fn ui() -> BoxedElement {
    WhCanvas::new(()).add_child(
        WhScreen::new(())
            .background(NodeBackground::Image("bg.png".into()))
            .direction(ElementDirection::Row, Val::Px(0.0))
            .add_child(
                WhDiv::new(())
                    .background(NodeBackground::Color(Color::rgba(0.0, 0.0, 0.0, 0.5)))
                    .direction(ElementDirection::Column, Val::Px(10.0)),
            ),
    )
}
