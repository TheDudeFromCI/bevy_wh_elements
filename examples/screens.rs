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
        .add_systems(Update, (show_screen_1, show_screen_2, show_screen_3))
        .run();
}

fn init(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    ui().build(&mut commands, &asset_server);
}

pub const SCREEN_1: ScreenID = ScreenID(1);
pub const SCREEN_2: ScreenID = ScreenID(2);
pub const SCREEN_3: ScreenID = ScreenID(3);
pub const SCREEN_GROUP: ScreenGroup = ScreenGroup(1);

fn ui() -> BoxedElement {
    WhCanvas::new().add_children(vec![
        WhScreen::new(SCREEN_1)
            .in_group(SCREEN_GROUP)
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
                        WhButton::with_flags(Screen2Button)
                            .bg_color(Color::WHITE)
                            .border(Color::BLACK, Val::Px(1.0))
                            .add_child(WhText::new().text("Show Screen 2").font_size(36.0)),
                        WhButton::with_flags(Screen3Button)
                            .bg_color(Color::WHITE)
                            .border(Color::BLACK, Val::Px(1.0))
                            .add_child(WhText::new().text("Show Screen 3").font_size(36.0)),
                    ]),
            ]),
        WhScreen::new(SCREEN_2)
            .in_group(SCREEN_GROUP)
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
                        WhButton::with_flags(Screen1Button)
                            .bg_color(Color::WHITE)
                            .border(Color::BLACK, Val::Px(1.0))
                            .add_child(WhText::new().text("Back").font_size(36.0)),
                    ]),
            ]),
        WhScreen::new(SCREEN_3)
            .in_group(SCREEN_GROUP)
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
                        WhButton::with_flags(Screen1Button)
                            .bg_color(Color::WHITE)
                            .border(Color::BLACK, Val::Px(1.0))
                            .add_child(WhText::new().text("Back").font_size(36.0)),
                    ]),
            ]),
    ])
}

#[derive(Component)]
pub struct Screen1Button;

#[derive(Component)]
pub struct Screen2Button;

#[derive(Component)]
pub struct Screen3Button;

fn show_screen_1(
    query_screens: Query<&Interaction, (With<Screen1Button>, Changed<Interaction>)>,
    mut toggle_screen_evs: EventWriter<SetScreenInGroup>,
) {
    for button_interaction in query_screens.iter() {
        if let Interaction::Pressed = button_interaction {
            toggle_screen_evs.send(SetScreenInGroup {
                screen_id: SCREEN_1,
                group: SCREEN_GROUP,
            });
        }
    }
}

fn show_screen_2(
    query_screens: Query<&Interaction, (With<Screen2Button>, Changed<Interaction>)>,
    mut toggle_screen_evs: EventWriter<SetScreenInGroup>,
) {
    for button_interaction in query_screens.iter() {
        if let Interaction::Pressed = button_interaction {
            toggle_screen_evs.send(SetScreenInGroup {
                screen_id: SCREEN_2,
                group: SCREEN_GROUP,
            });
        }
    }
}

fn show_screen_3(
    query_screens: Query<&Interaction, (With<Screen3Button>, Changed<Interaction>)>,
    mut toggle_screen_evs: EventWriter<SetScreenInGroup>,
) {
    for button_interaction in query_screens.iter() {
        if let Interaction::Pressed = button_interaction {
            toggle_screen_evs.send(SetScreenInGroup {
                screen_id: SCREEN_3,
                group: SCREEN_GROUP,
            });
        }
    }
}
