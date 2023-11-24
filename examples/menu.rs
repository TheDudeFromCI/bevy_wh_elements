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
        .add_systems(Update, (pressed_start, pressed_settings, pressed_quit))
        .run();
}

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct SettingsButton;

#[derive(Component)]
struct QuitButton;

fn init(asset_server: Res<AssetServer>, mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    ui().build(&mut commands, &asset_server);
}

fn pressed_start(ui: Query<&Interaction, (Changed<Interaction>, With<StartButton>)>) {
    for interaction in ui.iter() {
        if let Interaction::Pressed = interaction {
            println!("Pressed Start");
        }
    }
}

fn pressed_settings(ui: Query<&Interaction, (Changed<Interaction>, With<SettingsButton>)>) {
    for interaction in ui.iter() {
        if let Interaction::Pressed = interaction {
            println!("Pressed Settings");
        }
    }
}

fn pressed_quit(ui: Query<&Interaction, (Changed<Interaction>, With<QuitButton>)>) {
    for interaction in ui.iter() {
        if let Interaction::Pressed = interaction {
            println!("Pressed Quit");
        }
    }
}

fn ui() -> BoxedElement {
    WhCanvas::new() //
        .add_child(
            WhScreen::new(ScreenID(1))
                .bg_img("bg.png")
                .direction(ElementDirection::Row, Val::Px(0.0))
                .justify(ElementAlignment::Left)
                .padding(UiRect::all(Val::Px(5.0)))
                .add_child(
                    WhDiv::new()
                        .bg_color(Color::rgba(0.0, 0.0, 0.0, 0.5))
                        .direction(ElementDirection::Column, Val::Px(10.0))
                        .size(Val::Px(250.0), Val::Auto)
                        .padding(UiRect::all(Val::Px(10.0)))
                        .add_children(vec![
                            WhText::new()
                                .text("This header is extremely long a definitely won't fit in the box")
                                .border(Color::RED, Val::Px(2.0))
                                .size(Val::Percent(100.0), Val::Px(100.0)),
                            button(StartButton, "Start", ElementAlignment::Left),
                            button(SettingsButton, "Settings", ElementAlignment::Center),
                            button(QuitButton, "Quit", ElementAlignment::Right),
                            WhText::new()
                                .text("This footer is also extremely long but no_wrap is set to true")
                                .border(Color::RED, Val::Px(2.0))
                                .size(Val::Percent(100.0), Val::Px(100.0))
                                .no_wrap(),
                        ]),
                ),
        )
}

fn button(flags: impl Bundle, text: &str, align: ElementAlignment) -> BoxedElement {
    WhButton::with_flags(flags)
        .border(Color::WHITE, Val::Px(2.0))
        .size(Val::Percent(100.0), Val::Px(75.0))
        .add_child(
            WhText::new()
                .text(text)
                .size(Val::Percent(100.0), Val::Percent(100.0))
                .justify(align)
                .align(align)
                .padding(UiRect::all(Val::Px(5.0))),
        )
}
