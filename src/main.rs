use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy_rapier2d::prelude::*;
use std::process;
use std::fmt::Write;
use crate::player::PlayerPlugin;
use crate::animation::AnimationPlugin;
use crate::planet::PlanetPlugin;

mod player;
mod animation;
mod planet;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins
        .set(ImagePlugin::default_nearest()) // So pixel sprites won't render blurry.
        .set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Game Jam 5".into(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }),
    );
    app.add_plugins(LogDiagnosticsPlugin::default());
    app.add_plugins(FrameTimeDiagnosticsPlugin::default());
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0));
    app.add_plugins(RapierDebugRenderPlugin::default());
    app.add_plugins(PlayerPlugin);
    app.add_plugins(AnimationPlugin);
    app.add_plugins(PlanetPlugin);
    app.add_systems(Startup, setup);
    app.add_systems(Startup, spawn_fps_text);
    app.add_systems(Update, player_camera);
    app.add_systems(Update, update_fps_text);
    app.add_systems(Update, close_on_esc);

    app.run();
}

#[derive(Component)]
struct Camera;

fn setup(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default()).insert(Camera);
}

// Camera follows player, always centered on screen.
fn player_camera(
    player_q: Query<&Transform, With<player::Player>>,
    mut camera_q: Query<(&Camera, &mut Transform), Without<player::Player>>,
) {
    let player = player_q.single();
    let (_, mut camera) = camera_q.single_mut();

    // Set camera's coordinates to player's coordinates.
    camera.translation = player.translation;
    camera.rotation = player.rotation;
}


#[derive(Component)]
struct FrameRate;

// Spawn FPS in corner of screen.
fn spawn_fps_text(mut commands: Commands) {
    commands
        .spawn(TextBundle {
            text: Text::from_section(
                "FPS",
                TextStyle {
                    color: Color::WHITE,
                    ..default()
                },
            ),
            style: Style {
                display: Display::Flex,
                position_type: PositionType::Absolute,
                // Offset from top-left corner.
                top: Val::Px(10.),
                left: Val::Px(10.),
                ..default()
            },
            ..default()
        })
        .insert(FrameRate);
}

// Update FPS text on screen.
fn update_fps_text(
    mut text_query: Query<&mut Text, With<FrameRate>>,
    diagnostics: Res<DiagnosticsStore>,
) {
    let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|fps| fps.average())
    else {
        return;
    };

    for mut text in text_query.iter_mut() {
        let value = &mut text.sections[0].value;
        value.clear();

        write!(value, "FPS: {:.0}", fps).unwrap();
    }
}

// Exit app on ESC key press.
fn close_on_esc(
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        process::exit(0);
    }
}
