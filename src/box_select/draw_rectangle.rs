use std::time::Duration;

use bevy::prelude::*;
use rand::{rng, Rng};

pub struct DrawRectanglePlugin;
impl Plugin for DrawRectanglePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_rectangles);
    }
}

pub struct RectangleTestPlugin;

impl Plugin for RectangleTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_random_rectangles,
                cleanup_rectangles,
                move_rectangles,
                draw_rectangles,
            )
                .chain(),
        );
    }
}

#[derive(Component)]
pub struct DrawRectangle {
    pub start: Vec2,
    pub end: Vec2,
}

pub fn draw_rectangles(
    mut commands: Commands,
    query: Query<(Entity, &DrawRectangle)>,
) {
    for (entity, draw_rectangle) in query.iter() {
        // Calculate width, height and position from start/end points
        let width = (draw_rectangle.end.x - draw_rectangle.start.x).abs();
        let height = (draw_rectangle.end.y - draw_rectangle.start.y).abs();

        // Get top-left position
        let left = draw_rectangle.start.x.min(draw_rectangle.end.x);
        let top = draw_rectangle.start.y.min(draw_rectangle.end.y);

        commands
            .entity(entity)
            .insert(Node {
                // Set position type to absolute for precise positioning
                position_type: PositionType::Absolute,

                // Set position using the calculated values
                left: Val::Px(left),
                top: Val::Px(top),

                // Set size using the calculated width/height
                width: Val::Px(width),
                height: Val::Px(height),

                // Add border
                border: UiRect::all(Val::Px(1.0)),

                // Use default values for other properties
                ..default()
            })
            .insert(BackgroundColor(Color::linear_rgba(0.8, 0.8, 0.8, 0.15))) // Transparent background
            .insert(BorderColor(Color::linear_rgba(0.8, 0.8, 0.8, 1.0))); // Solid border
    }
}

// Component to track lifetime of rectangles
#[derive(Component)]
pub struct RectangleTimer(Timer);

// System to randomly create new rectangles
pub fn spawn_random_rectangles(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: Local<Timer>,
) {
    // Initialize timer if needed
    if spawn_timer.duration() == Duration::ZERO {
        *spawn_timer =
            Timer::new(Duration::from_secs_f32(2.0), TimerMode::Repeating);
    }

    spawn_timer.tick(time.delta());

    if spawn_timer.just_finished() {
        let mut rng = rng();

        // Random position within 800x600 area
        let start = Vec2::new(
            rng.random_range(0.0..800.0),
            rng.random_range(0.0..600.0),
        );
        let end = Vec2::new(
            rng.random_range(0.0..800.0),
            rng.random_range(0.0..600.0),
        );

        // Random lifetime between 3-8 seconds
        let lifetime = rng.random_range(0.5..3.0);

        commands.spawn((
            DrawRectangle { start, end },
            RectangleTimer(Timer::from_seconds(lifetime, TimerMode::Once)),
        ));
    }
}

// System to remove rectangles after their lifetime
pub fn cleanup_rectangles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut RectangleTimer)>,
) {
    for (entity, mut timer) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            commands.entity(entity).despawn();
        }
    }
}

// System to move existing rectangles
pub fn move_rectangles(time: Res<Time>, mut query: Query<&mut DrawRectangle>) {
    let mut rng = rng();
    let speed = 100.0; // pixels per second

    for mut rectangle in query.iter_mut() {
        // Random movement direction
        let time_secs = time.elapsed_secs();
        let movement = Vec2::new(time_secs.sin(), time_secs.cos()).normalize()
            * speed
            * time.delta_secs();
        let movement2 = Vec2::new(time_secs.cos(), time_secs.sin()).normalize()
            * speed
            * time.delta_secs();

        // Move both start and end points
        rectangle.start += movement;
        rectangle.end += movement2;

        // Keep rectangles within bounds (800x600)
        rectangle.start.x = rectangle.start.x.clamp(0.0, 800.0);
        rectangle.start.y = rectangle.start.y.clamp(0.0, 600.0);
        rectangle.end.x = rectangle.end.x.clamp(0.0, 800.0);
        rectangle.end.y = rectangle.end.y.clamp(0.0, 600.0);
    }
}

// Plugin to set up the test systems
