use bevy::prelude::*;
use ownership::player::OwnedBy;

use super::mouse_drag::{MouseDragFinished, MouseDragListener};

// Components
#[derive(Component)]
#[require(Camera, MouseDragListener)]
pub struct Selector;

#[derive(Component)]
pub struct Selectable;

#[derive(Component, Default)]
pub struct Selections(pub Vec<Entity>);

pub struct UnitSelectionPlugin;

impl Plugin for UnitSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, add_selections);
    }
}

fn add_selections(
    mut commands: Commands,
    // Single query for camera with both Selector and MouseDragFinished
    selector_camera_q: Query<
        (
            Entity,
            &Camera,
            &GlobalTransform,
            &MouseDragFinished,
            &OwnedBy,
        ),
        With<Selector>,
    >,
    selectable_q: Query<(Entity, &GlobalTransform), With<Selectable>>,
) {
    // Process for each camera that has finished a drag (usually will be just one)
    for (camera_entity, camera, camera_transform, drag, owner) in
        selector_camera_q.iter()
    {
        let min_x = drag.start.x.min(drag.end.x);
        let max_x = drag.start.x.max(drag.end.x);
        let min_y = drag.start.y.min(drag.end.y);
        let max_y = drag.start.y.max(drag.end.y);

        let mut selected_entities = Vec::new();

        for (entity, transform) in selectable_q.iter() {
            if let Ok(screen_pos) = camera
                .world_to_viewport(camera_transform, transform.translation())
            {
                if screen_pos.x >= min_x
                    && screen_pos.x <= max_x
                    && screen_pos.y >= min_y
                    && screen_pos.y <= max_y
                {
                    selected_entities.push(entity);
                }
            }
        }

        commands.entity(camera_entity).remove::<MouseDragFinished>();
        commands
            .entity(owner.entity)
            .insert(Selections(selected_entities));
    }
}

// Resource to hold the shared mesh and material for highlights
#[derive(Resource)]
struct HighlightAssets {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

pub struct UnitSelectionTestPlugin;

impl Plugin for UnitSelectionTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_highlight_assets)
            .add_systems(Update, highlight_selected_units);
    }
}

fn setup_highlight_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let highlight_assets = HighlightAssets {
        mesh: meshes.add(Cuboid::default()),
        material: materials.add(StandardMaterial {
            base_color: Color::linear_rgba(0.0, 1.0, 0.0, 0.5),
            alpha_mode: AlphaMode::Blend,
            ..default()
        }),
    };

    commands.insert_resource(highlight_assets);
}

fn highlight_selected_units(
    mut commands: Commands,
    highlight_assets: Res<HighlightAssets>,
    selection_q: Query<&Selections>,
    old_highlights: Query<Entity, With<SelectionHighlight>>,
    selected_transforms: Query<&GlobalTransform, With<Selectable>>,
) {
    for entity in old_highlights.iter() {
        commands.entity(entity).despawn();
    }

    for selections in selection_q.iter() {
        for &selected_entity in selections.0.iter() {
            if let Ok(transform) = selected_transforms.get(selected_entity) {
                commands.spawn((
                    Mesh3d(highlight_assets.mesh.clone()),
                    MeshMaterial3d(highlight_assets.material.clone()),
                    Transform::from_translation(transform.translation())
                        .with_scale(Vec3::splat(1.2)),
                    SelectionHighlight,
                ));
            }
        }
    }
}

#[derive(Component)]
struct SelectionHighlight;
