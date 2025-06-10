use bevy::input::mouse::MouseButton;
use bevy::prelude::*;

use super::draw_rectangle::DrawRectangle;

// Components
#[derive(Component, Default)]
pub struct MouseDragListener;

#[derive(Component)]
pub struct MouseDragStart(pub Vec2);

#[derive(Component)]
pub struct MouseDragNow(pub Vec2);

#[derive(Component)]
pub struct MouseDragFinished {
    pub start: Vec2,
    pub end: Vec2,
}

// Plugin
pub struct MouseDragPlugin;

impl Plugin for MouseDragPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                put_mouse_drag_start_on_listeners,
                put_mouse_drag_now_on_listeners,
                finish_mouse_drag,
            )
                .chain(),
        );
    }
}

pub fn put_mouse_drag_start_on_listeners(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    query: Query<Entity, (With<MouseDragListener>, Without<MouseDragStart>)>,
) {
    let window = match windows.get_single() {
        Ok(window) => window,
        Err(_) => return,
    };

    if mouse_button.just_pressed(MouseButton::Left) {
        if let Some(position) = window.cursor_position() {
            for entity in query.iter() {
                commands.entity(entity).insert(MouseDragStart(position));
            }
        }
    }
}

pub fn put_mouse_drag_now_on_listeners(
    mut commands: Commands,
    windows: Query<&Window>,
    query: Query<Entity, (With<MouseDragStart>, With<MouseDragListener>)>,
) {
    if let Ok(window) = windows.get_single() {
        if let Some(position) = window.cursor_position() {
            for entity in query.iter() {
                commands.entity(entity).insert(MouseDragNow(position));
            }
        }
    }
}

pub fn finish_mouse_drag(
    mut commands: Commands,
    mouse_button: Res<ButtonInput<MouseButton>>,
    query: Query<
        (Entity, &MouseDragStart, Option<&MouseDragNow>),
        With<MouseDragListener>,
        
    >,
) {
    if mouse_button.just_released(MouseButton::Left) {
        for (entity, start, now) in query.iter() {
            let end_pos = now.map_or(start.0, |now| now.0);

            commands.entity(entity).insert(MouseDragFinished {
                start: start.0,
                end: end_pos,
            });

            commands
                .entity(entity)
                .remove::<MouseDragStart>()
                .remove::<MouseDragNow>();
        }
    }
}
pub struct MouseDragRectangleTestPlugin;

impl Plugin for MouseDragRectangleTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_drag_test)
            .add_systems(Update, update_drag_rectangle)
            .add_systems(Update, reset_drag_rectangle);
    }
}

fn setup_drag_test(mut commands: Commands) {
    commands.spawn((
        MouseDragListener,
        DrawRectangle {
            start: Vec2::ZERO,
            end: Vec2::ZERO,
        },
    ));
}

fn update_drag_rectangle(
    mut rectangle_query: Query<&mut DrawRectangle>,
    drag_query: Query<(&MouseDragStart, Option<&MouseDragNow>)>,
) {
    for mut draw_rect in rectangle_query.iter_mut() {
        for (start, now) in drag_query.iter() {
            draw_rect.start = start.0;
            draw_rect.end = if let Some(now) = now { now.0 } else { start.0 };
        }
    }
}
fn reset_drag_rectangle(
    mut commands: Commands,
    mut rectangle_query: Query<(
        Entity,
        &mut DrawRectangle,
        &MouseDragFinished,
    )>,
) {
    for (entity, mut draw_rect, _) in rectangle_query.iter_mut() {
        draw_rect.start = Vec2::ZERO;
        draw_rect.end = Vec2::ZERO;
        commands.entity(entity).remove::<MouseDragFinished>();
    }
}
