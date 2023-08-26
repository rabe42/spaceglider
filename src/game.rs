use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn setup(mut commands: Commands,
         meshes: ResMut<Assets<Mesh>>,
         materials: ResMut<Assets<ColorMaterial>>,
         asset_server: Res<AssetServer>,
) {
    // The camera for the HUD
    commands.spawn(Camera2dBundle::default());

    // Creating some static text to be displayed on the HUD
    commands.spawn(TextBundle::from_section(
        "Hold 'Left' or 'Right' to change the line width",
        TextStyle {
            font: asset_server.load("fonts/Gruppo-Regular.ttf"),
            font_size: 24.,
            color: Color::WHITE,
        },
    ));

    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>)
{
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {}!", name.0);
        }
    }
}

fn update_hud(
    mut gizmos: Gizmos,
    time: Res<Time>
) {
    let sin = time.elapsed_seconds().sin() * 50.;
    gizmos.line_2d(Vec2::Y * -sin, Vec2::splat(-80.), Color::RED);
    gizmos.ray_2d(Vec2::Y * sin, Vec2::splat(80.), Color::GREEN);

    // Triangle
    gizmos.linestrip_gradient_2d([
        (Vec2::Y * 300., Color::BLUE),
        (Vec2::new(-255., -155.), Color::RED),
        (Vec2::new(255., -155.), Color::GREEN),
        (Vec2::Y * 300., Color::BLUE),
    ]);

    gizmos.rect_2d(
        Vec2::ZERO,
        time.elapsed_seconds() / 3.,
        Vec2::splat(300.),
        Color::BLACK,
    );

    // The circles have 32 line-segments by default.
    gizmos.circle_2d(Vec2::ZERO, 120., Color::BLACK);
    // You may want to increase this for larger circles.
    gizmos.circle_2d(Vec2::ZERO, 300., Color::NAVY).segments(64);

    // Arcs default amount of segments is linerarly interpolated between
    // 1 and 32, using the arc length as scalar.
    gizmos.arc_2d(Vec2::ZERO, sin / 10., PI / 2., 350., Color::ORANGE_RED);
}

/// This is the basic plugin for the Spaceglider game. In the first step, I try to setup the head
/// up dsiplay (HUD).
pub struct Spaceglider;

impl Plugin for Spaceglider {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            // Setting the background color
            .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
            // Initializes the system
            .add_systems(Startup, setup)
            // Start the interaction
            .add_systems(Update, (update_hud, greet_people));
    }
}
