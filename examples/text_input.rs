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
        WhScreen::new(()) //
            .bg_img("bg.png")
            .direction(ElementDirection::Row, Val::Px(100.0))
            .add_children(vec![
                WhDiv::new(())
                    .direction(ElementDirection::Column, Val::Px(0.0))
                    .bg_color(Color::rgba(0.0, 0.0, 0.0, 0.5))
                    .border(Color::WHITE, Val::Px(1.0))
                    .padding(UiRect::all(Val::Px(10.0)))
                    .add_children(vec![
                        WhText::new((), ()) //
                            .size(Val::Percent(100.0), Val::Px(20.0))
                            .text("Username:")
                            .font_size(16.0)
                            .text_color(Color::WHITE),
                        WhTextInput::new((), ())
                            .bg_color(Color::WHITE)
                            .border(Color::DARK_GRAY, Val::Px(2.0))
                            .size(Val::Percent(100.0), Val::Px(30.0))
                            .margin(UiRect::bottom(Val::Px(10.0)))
                            .text("Please enter username")
                            .no_wrap(),
                        WhText::new((), ()) //
                            .size(Val::Percent(100.0), Val::Px(20.0))
                            .text("Password:")
                            .font_size(16.0)
                            .text_color(Color::WHITE),
                        WhTextInput::new((), ())
                            .bg_color(Color::WHITE)
                            .border(Color::DARK_GRAY, Val::Px(2.0))
                            .size(Val::Percent(100.0), Val::Px(30.0))
                            .margin(UiRect::bottom(Val::Px(10.0)))
                            .text("Please enter password")
                            .no_wrap(),
                        WhDiv::new(())
                            .direction(ElementDirection::Row, Val::Px(10.0))
                            .add_children(vec![
                                WhButton::new(())
                                    .bg_color(Color::rgb(0.8, 0.8, 0.8))
                                    .border(Color::DARK_GRAY, Val::Px(2.0))
                                    .size(Val::Px(200.0), Val::Px(30.0))
                                    .add_child(WhText::new((), ()).text("Login")),
                                WhButton::new(())
                                    .bg_color(Color::rgb(0.8, 0.8, 0.8))
                                    .border(Color::DARK_GRAY, Val::Px(2.0))
                                    .size(Val::Px(200.0), Val::Px(30.0))
                                    .add_child(WhText::new((), ()).text("Cancel")),
                            ]),
                    ]),
                WhDiv::new(())
                    .direction(ElementDirection::Column, Val::Px(0.0))
                    .bg_color(Color::rgba(0.0, 0.0, 0.0, 0.5))
                    .border(Color::WHITE, Val::Px(1.0))
                    .padding(UiRect::all(Val::Px(10.0)))
                    .add_children(vec![
                        WhText::new((), ()) //
                            .size(Val::Percent(100.0), Val::Px(20.0))
                            .text("Review:")
                            .font_size(16.0)
                            .text_color(Color::WHITE),
                        WhTextInput::new((), ())
                            .bg_color(Color::WHITE)
                            .border(Color::DARK_GRAY, Val::Px(2.0))
                            .size(Val::Percent(100.0), Val::Px(200.0))
                            .margin(UiRect::bottom(Val::Px(10.0)))
                            .text("Please enter review")
                            .justify(ElementAlignment::Left)
                            .align(ElementAlignment::Top),
                        WhDiv::new(())
                            .direction(ElementDirection::Row, Val::Px(10.0))
                            .add_children(vec![
                                WhButton::new(())
                                    .bg_color(Color::rgb(0.8, 0.8, 0.8))
                                    .border(Color::DARK_GRAY, Val::Px(2.0))
                                    .size(Val::Px(200.0), Val::Px(30.0))
                                    .add_child(WhText::new((), ()).text("Submit")),
                                WhButton::new(())
                                    .bg_color(Color::rgb(0.8, 0.8, 0.8))
                                    .border(Color::DARK_GRAY, Val::Px(2.0))
                                    .size(Val::Px(200.0), Val::Px(30.0))
                                    .add_child(WhText::new((), ()).text("Cancel")),
                            ]),
                    ]),
            ]),
    )
}
