//! The Bevy demo: the letters of "TELETYPE GAMES" ride a sine wave with drop
//! shadows, cycling through a five-colour palette.

use bevy::prelude::*;

/// Window width in pixels.
const SCREEN_W: f32 = 640.0;
/// Window height in pixels.
const SCREEN_H: f32 = 480.0;
/// The text that waves.
const MESSAGE: &str = "TELETYPE GAMES";
/// Horizontal distance between two letters, in pixels.
const CHAR_W: f32 = 40.0;
/// Height of the wave, in pixels up and down.
const WAVE_AMPLITUDE: f32 = 28.0;

/// Returns colour `i` of the five-colour palette, wrapping around.
fn palette(i: usize) -> Color {
    match i % 5 {
        0 => Color::srgb_u8(0x1a, 0x1c, 0x2c),
        1 => Color::srgb_u8(0x5d, 0x27, 0x5d),
        2 => Color::srgb_u8(0xb1, 0x3e, 0x53),
        3 => Color::srgb_u8(0xef, 0x7d, 0x57),
        _ => Color::srgb_u8(0xff, 0xcd, 0x75),
    }
}

/// A letter (or its shadow), with its index in [`MESSAGE`].
#[derive(Component)]
struct Letter(usize);

/// Marks the shadow copy of a letter.
#[derive(Component)]
struct Shadow;

/// Opens the window and runs the app with the [`setup`] and [`animate`] systems.
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Teletype Games".into(),
                resolution: (SCREEN_W, SCREEN_H).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup)
        .add_systems(Update, animate)
        .run();
}

/// Spawns the camera and, for every letter, its shadow and the letter itself.
fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    let total_w = MESSAGE.chars().count() as f32 * CHAR_W;
    for (i, ch) in MESSAGE.chars().enumerate() {
        let x = i as f32 * CHAR_W - total_w / 2.0 + CHAR_W / 2.0;
        let font = TextFont {
            font_size: 48.0,
            ..default()
        };

        commands.spawn((
            Text2d::new(ch.to_string()),
            font.clone(),
            TextColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            Transform::from_xyz(x + 4.0, -4.0, 0.0),
            Letter(i),
            Shadow,
        ));
        commands.spawn((
            Text2d::new(ch.to_string()),
            font,
            TextColor(palette(i)),
            Transform::from_xyz(x, 0.0, 1.0),
            Letter(i),
        ));
    }
}

/// Moves every letter along the wave and shifts the letters' colours over time;
/// a shadow follows its letter 4 pixels lower.
fn animate(
    time: Res<Time>,
    mut query: Query<(&Letter, &mut Transform, &mut TextColor, Option<&Shadow>)>,
) {
    let t = time.elapsed_secs() * 2.5;
    for (letter, mut transform, mut color, shadow) in &mut query {
        let y = (t + letter.0 as f32 * 0.4).sin() * WAVE_AMPLITUDE;
        if shadow.is_some() {
            transform.translation.y = y - 4.0;
        } else {
            transform.translation.y = y;
            color.0 = palette(letter.0 + (t * 0.8) as usize);
        }
    }
}
