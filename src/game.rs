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

    // Creating some Lines to be displayed on the HUD
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

pub struct Spaceglider;

impl Plugin for Spaceglider {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            // Setting the background color
            .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
            // Initializes the system
            .add_systems(Startup, setup)
            // Start the interaction
            .add_systems(Update, greet_people);
    }
}
