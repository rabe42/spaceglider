use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Component)]
struct ExampleDisplay;

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

// Note: Everything, which should be displayed must be a component or a resource. 
// The magic happens in the function parameter lists, where an arbitrary number of Queries on
// Resources (Res or ResMut) and Components (Query) may be executed.

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

/// Setup the different game parts.
fn setup(mut commands: Commands,
         _meshes: ResMut<Assets<Mesh>>,
         _materials: ResMut<Assets<ColorMaterial>>,
         asset_server: Res<AssetServer>,
) {
    // The camera for the HUD
    commands.spawn(Camera2dBundle::default());

    // Creating some static text to be displayed on the HUD
    commands.spawn(TextBundle::from_section(
        "© Dr. Ralf Berger",
        TextStyle {
            font: asset_server.load("fonts/Gruppo-Regular.ttf"),
            font_size: 12.,
            color: Color::WHITE,
        },
    ));

    let display_text_style = TextStyle {
        font: asset_server.load("fonts/Gruppo-Regular.ttf"),
        font_size: 22.,
        color: Color::BLACK,
    };

    // Create a area, where we will write dynamic text onto.
    // Note: We spawn a tuple here!
    commands.spawn((
        TextBundle::from_section("", display_text_style)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.),
            right: Val::Px(20.0),
            ..default()
        }),
        ExampleDisplay,
    ));

    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

/// Just demonstrate, that we are still alive.
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

/// Update the HUD with status data from the ships model and the space environement.
fn update_hud(
    mut gizmos: Gizmos,
    mut display: Query<&mut Text, With<ExampleDisplay>>,
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

    // Write some information on the screen
    // Create a cross hair
    let mut display = display.single_mut();
    display.sections[0].value = format!("Distance: {}\nObject Type: {}", time.elapsed_seconds(), "Hypo");
}

