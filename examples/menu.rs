use bevy::prelude::*;
use bevy_wh_elements::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
    WhCanvas::new(()).add_child(
        WhScreen::new(())
            .background(NodeBackground::Image("bg.png".into()))
            .direction(ElementDirection::Row, Val::Px(0.0))
            .justify(ElementAlignment::Start)
            .padding(UiRect::all(Val::Px(5.0)))
            .add_child(
                WhDiv::new(())
                    .background(NodeBackground::Color(Color::rgba(0.0, 0.0, 0.0, 0.5)))
                    .direction(ElementDirection::Column, Val::Px(10.0))
                    .size(Val::Px(250.0), Val::Auto)
                    .padding(UiRect::all(Val::Px(10.0)))
                    .add_children(vec![
                        button(StartButton, "Start", ElementAlignment::Start),
                        button(SettingsButton, "Settings", ElementAlignment::Center),
                        button(QuitButton, "Quit", ElementAlignment::End),
                    ]),
            ),
    )
}

fn button(flags: impl Bundle, text: &str, align: ElementAlignment) -> BoxedElement {
    WhButton::new(flags)
        .background(NodeBackground::Bordered {
            bg: Color::NONE,
            border: Color::WHITE,
            thickness: Val::Px(2.0),
        })
        .size(Val::Percent(100.0), Val::Px(75.0))
        .add_child(
            WhText::new(())
                .text(text)
                .background(NodeBackground::Color(Color::RED))
                .direction(ElementDirection::Row, Val::Px(0.0))
                .justify(align)
                .align(align)
                .no_wrap()
                .padding(UiRect::all(Val::Px(5.0))),
        )
}
