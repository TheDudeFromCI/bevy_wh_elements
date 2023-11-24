use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_wh_elements::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(LogPlugin {
            level: bevy::log::Level::DEBUG,
            ..default()
        }))
        .add_plugins(WhElementsPlugin)
        .add_systems(Startup, init)
        .run();
}

fn init(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    ui().build(&mut commands, &asset_server);
}

pub const SCREEN_1: ScreenID = ScreenID(1);
pub const SCREEN_2: ScreenID = ScreenID(2);
pub const SCREEN_3: ScreenID = ScreenID(3);
pub const SCREEN_4: ScreenID = ScreenID(4);
pub const SCREEN_GROUP_1: ScreenGroup = ScreenGroup(1);
pub const SCREEN_GROUP_2: ScreenGroup = ScreenGroup(2);

fn ui() -> BoxedElement {
    WhCanvas::new().add_children(vec![
        WhScreen::new(SCREEN_1)
            .in_group(SCREEN_GROUP_1)
            .bg_img("bg.png")
            .shown()
            .add_children(vec![
                WhDiv::new()
                    .bg_color(Color::rgba(0.0, 0.0, 0.0, 0.5))
                    .border(Color::WHITE, Val::Px(2.0))
                    .direction(ElementDirection::Column, Val::Px(10.0))
                    .padding(UiRect::all(Val::Px(10.0)))
                    .add_children(vec![
                        WhText::new()
                            .text("Screen 1")
                            .font_size(36.0)
                            .text_color(Color::WHITE)
                            .margin(UiRect::all(Val::Px(20.0))),
                        WhButton::new()
                            .bg_color(Color::WHITE)
                            .border(Color::BLACK, Val::Px(1.0))
                            .padding(UiRect::all(Val::Px(5.0)))
                            .on_click(SetScreenInGroup::new(SCREEN_GROUP_1, SCREEN_2))
                            .add_child(WhText::new().text("Show Screen 2").font_size(36.0)),
                        WhButton::new()
                            .bg_color(Color::WHITE)
                            .border(Color::BLACK, Val::Px(1.0))
                            .padding(UiRect::all(Val::Px(5.0)))
                            .on_click(SetScreenInGroup::new(SCREEN_GROUP_1, SCREEN_3))
                            .add_child(WhText::new().text("Show Screen 3").font_size(36.0)),
                        WhButton::new()
                            .bg_color(Color::WHITE)
                            .border(Color::BLACK, Val::Px(1.0))
                            .padding(UiRect::all(Val::Px(5.0)))
                            .on_click(SetScreenVisible::show(SCREEN_4))
                            .add_child(WhText::new().text("Show Screen 4").font_size(36.0)),
                    ]),
            ]),
        WhScreen::new(SCREEN_2)
            .in_group(SCREEN_GROUP_1)
            .bg_img("bg2.png")
            .add_children(vec![
                WhDiv::new()
                    .bg_color(Color::rgba(0.0, 0.0, 0.0, 0.5))
                    .border(Color::WHITE, Val::Px(2.0))
                    .direction(ElementDirection::Column, Val::Px(10.0))
                    .padding(UiRect::all(Val::Px(10.0)))
                    .add_children(vec![
                        WhText::new()
                            .text("Screen 2")
                            .font_size(36.0)
                            .text_color(Color::WHITE)
                            .margin(UiRect::all(Val::Px(20.0))),
                        WhButton::new()
                            .bg_color(Color::WHITE)
                            .border(Color::BLACK, Val::Px(1.0))
                            .padding(UiRect::all(Val::Px(5.0)))
                            .on_click(SetScreenVisible::show(SCREEN_4))
                            .add_child(WhText::new().text("Show Screen 4").font_size(36.0)),
                        WhButton::new()
                            .bg_color(Color::WHITE)
                            .border(Color::BLACK, Val::Px(1.0))
                            .padding(UiRect::all(Val::Px(5.0)))
                            .on_click(SetScreenInGroup::new(SCREEN_GROUP_1, SCREEN_1))
                            .add_child(WhText::new().text("Back").font_size(36.0)),
                    ]),
            ]),
        WhScreen::new(SCREEN_3)
            .in_group(SCREEN_GROUP_1)
            .bg_img("bg3.png")
            .add_children(vec![
                WhDiv::new()
                    .bg_color(Color::rgba(0.0, 0.0, 0.0, 0.5))
                    .border(Color::WHITE, Val::Px(2.0))
                    .direction(ElementDirection::Column, Val::Px(10.0))
                    .padding(UiRect::all(Val::Px(10.0)))
                    .add_children(vec![
                        WhText::new()
                            .text("Screen 3")
                            .font_size(36.0)
                            .text_color(Color::WHITE)
                            .margin(UiRect::all(Val::Px(20.0))),
                        WhButton::new()
                            .bg_color(Color::WHITE)
                            .border(Color::BLACK, Val::Px(1.0))
                            .padding(UiRect::all(Val::Px(5.0)))
                            .on_click(SetScreenVisible::show(SCREEN_4))
                            .add_child(WhText::new().text("Show Screen 4").font_size(36.0)),
                        WhButton::new()
                            .bg_color(Color::WHITE)
                            .border(Color::BLACK, Val::Px(1.0))
                            .padding(UiRect::all(Val::Px(5.0)))
                            .on_click(SetScreenInGroup::new(SCREEN_GROUP_1, SCREEN_1))
                            .add_child(WhText::new().text("Back").font_size(36.0)),
                    ]),
            ]),
        WhScreen::new(SCREEN_4)
            .in_group(SCREEN_GROUP_2)
            .justify(ElementAlignment::Right)
            .align(ElementAlignment::Top)
            .add_children(vec![
                WhDiv::new()
                    .bg_color(Color::rgba(0.0, 0.0, 0.0, 0.5))
                    .border(Color::WHITE, Val::Px(2.0))
                    .direction(ElementDirection::Column, Val::Px(10.0))
                    .padding(UiRect::all(Val::Px(10.0)))
                    .add_children(vec![
                        WhText::new()
                            .text("Screen 4\nToggle me!")
                            .font_size(42.0)
                            .text_color(Color::WHITE)
                            .margin(UiRect::all(Val::Px(20.0))),
                        WhButton::new()
                            .bg_color(Color::WHITE)
                            .border(Color::BLACK, Val::Px(1.0))
                            .padding(UiRect::all(Val::Px(5.0)))
                            .on_click(SetScreenVisible::hide(SCREEN_4))
                            .add_child(WhText::new().text("Hide Me").font_size(36.0)),
                    ]),
            ]),
    ])
}
