use bevy::prelude::*;

#[derive(Component)]
pub struct SpawnCube {
    pub color: Color,
}

pub fn spawn_cube(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &SpawnCube)>,
) {
    for (entity, cube) in query.iter() {
        let cube_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));
        let cube_material = materials.add(cube.color);
        commands
            .entity(entity)
            .insert((Mesh3d(cube_mesh), MeshMaterial3d(cube_material)));
    }
}
