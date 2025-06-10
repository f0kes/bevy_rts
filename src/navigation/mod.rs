use std::time::Duration;

use avian3d::{math::*, prelude::*};
use bevy::{prelude::*, time::common_conditions::on_timer};
use movement::constraint::MovementConstraint;
use vleue_navigator::prelude::*;

use crate::dudliq::Destination;

#[derive(Component)]
pub struct Obstacle;

pub struct NavigationPlugin {
    pub debug: bool,
}
fn setup_gizmo_config(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<PathGizmos>();

    // Line appearance
    config.line_width = 2.0;
    //config.line_perspective = true;

    //config.line_style = GizmoLineStyle::Solid;
    //config.line_joints = GizmoLineJoint::Round(8);
    // Rendering
    //config.depth_bias = -0.1; // Draw on top
    config.enabled = true;
}

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            VleueNavigatorPlugin,
            NavmeshUpdaterPlugin::<Collider, Obstacle>::default(),
        ))
        .init_resource::<NavMeshConstraint>()
        .init_gizmo_group::<PathGizmos>() // Initialize gizmo group
        .add_systems(
            Update,
            (
                check_if_need_reconstruction,
                on_new_navigation,
                reconstruct_path,
            )
                .chain(),
        )
        .add_systems(Update, update_navmesh_constraint);

        if self.debug {
            app.add_systems(
                Update,
                (
                    view_navmesh.run_if(on_timer(Duration::from_secs_f32(1.0))),
                    draw_paths, // Add path visualization
                ),
            );
            app.add_systems(Startup, setup_gizmo_config);

            // Configure path gizmos
        }
    }
}

fn view_navmesh(
    mut commands: Commands,
    navmeshes: Query<Entity, With<ManagedNavMesh>>,
    mut current: Local<usize>,
) {
    for (i, entity) in navmeshes.iter().sort::<Entity>().enumerate() {
        commands.entity(entity).remove::<NavMeshDebug>();
        if i == *current {
            commands
                .entity(entity)
                .insert(NavMeshDebug(Color::srgba(1.0, 0.0, 0.0, 0.8)));
        }
    }
    *current = (*current + 1) % navmeshes.iter().len();
}

#[derive(Component)]
pub struct NavigateTo(pub Vec3); // The final destination we want to reach

#[derive(Component)]
pub struct ReconstructPath;

pub fn check_if_need_reconstruction(
    mut commands: Commands,
    units: Query<(Entity, &Transform, &NavigateTo, Option<&Destination>)>,
) {
    for (entity, transform, nav_to, dest) in units.iter() {
        match dest {
            Some(dest) => {
                let dist_to_current =
                    (dest.position - transform.translation).length_squared();
                let dist_to_final =
                    (nav_to.0 - transform.translation).length_squared();

                if dist_to_current < 1.0 && dist_to_final > 1.0 {
                    commands.entity(entity).insert(ReconstructPath);
                }
            }
            None => {
                commands.entity(entity).insert(ReconstructPath);
            }
        }
    }
}

pub fn on_new_navigation(
    mut commands: Commands,
    new_navigations: Query<Entity, Changed<NavigateTo>>,
) {
    for entity in new_navigations.iter() {
        commands.entity(entity).insert(ReconstructPath);
    }
}

pub fn reconstruct_path(
    mut commands: Commands,
    navmeshes: Res<Assets<NavMesh>>,
    navmesh_query: Query<&ManagedNavMesh>,
    units: Query<(Entity, &Transform, &NavigateTo), With<ReconstructPath>>,
) {
    let Some(navmesh) = navmeshes.get(navmesh_query.single()) else {
        return;
    };

    for (entity, transform, nav_to) in units.iter() {
        if let Some(path) =
            navmesh.transformed_path(transform.translation, nav_to.0)
        {
            let tolerance = if path.path.len() == 1 { 1.0 } else { 0.0 };
            if let Some(next_point) = path.path.first() {
                commands
                    .entity(entity)
                    .insert(Destination {
                        position: *next_point,
                        tolerance,
                    })
                    .remove::<ReconstructPath>();
            }
        }
    }
}

#[derive(Resource, Default)]
pub struct NavMeshConstraint {
    navmesh: Option<NavMesh>,
}

impl MovementConstraint for NavMeshConstraint {
    fn is_move_allowed(&self, from: Vec3, to: Vec3) -> bool {
        let _ = from;
        if let Some(navmesh) = &self.navmesh {
            navmesh.is_in_mesh(to.xz())
        } else {
            false
        }
    }
}
pub fn update_navmesh_constraint(
    navmeshes: Res<Assets<NavMesh>>,
    navmesh_query: Query<&ManagedNavMesh>,
    mut constraint: ResMut<NavMeshConstraint>,
) {
    if let Some(navmesh) = navmeshes.get(navmesh_query.single()) {
        constraint.navmesh = Some(navmesh.clone());
    } else {
        constraint.navmesh = None;
    }
}
#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct PathGizmos;

pub fn draw_paths(
    navmeshes: Res<Assets<NavMesh>>,
    navmesh_query: Query<&ManagedNavMesh>,
    path_entities: Query<(&Transform, &NavigateTo)>,
    mut gizmos: Gizmos<PathGizmos>,
) {
    let Some(navmesh) = navmeshes.get(navmesh_query.single()) else {
        return;
    };

    for (transform, nav_to) in path_entities.iter() {
        gizmos.sphere(
            transform.translation,
            0.2,
            Color::srgba(0.0, 1.0, 0.0, 1.0),
        );
        gizmos.sphere(nav_to.0, 0.2, Color::srgba(1.0, 0.0, 0.0, 1.0));
        if let Some(path) =
            navmesh.transformed_path(transform.translation, nav_to.0)
        {
            gizmos.line(
                transform.translation,
                path.path[0],
                Color::srgba(0.0, 0.8, 1.0, 0.8),
            );
            for points in path.path.windows(2) {
                gizmos.line(
                    points[0],
                    points[1],
                    Color::srgba(0.0, 0.8, 1.0, 0.8),
                );
            }

            for point in path.path.iter() {
                gizmos.sphere(*point, 0.1, Color::srgba(1.0, 1.0, 0.0, 0.8));
            }
        }
    }
}
