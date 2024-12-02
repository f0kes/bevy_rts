use bevy::{ecs::query, prelude::*, utils::HashMap};
use misc::disabled::ToggleCommands;
use movement::{
    kinematic_character_controller::KinematicCharacterControllerBundle,
    movement::GlueToGround,
    rotate::{RotateInDirectionOfMovement, TiltInDirectionOfMovement},
    step_animation::StepAnimation,
};
use outline::{
    material_replace::ReplaceMaterialKeepTextureMarker,
    toon_shader::{default_toon_shader_material, ToonShaderMaterial},
};
use steering::plugin::SteeringAgent;
#[derive(Component, Clone, Copy, Debug)]
pub struct Unit {
    pub unit_name: UnitName,
    pub stack_size: u32,
    pub model_path: &'static str,
    pub spawn_offset: Vec3,
}
impl Default for Unit {
    fn default() -> Self {
        Self {
            unit_name: UnitName::None,
            stack_size: 16,
            model_path: "models/dudliq.glb#Scene0",
            spawn_offset: Vec3 {
                x: 0.,
                y: 0.45,
                z: 0.,
            },
        }
    }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum UnitName {
    None,
    Dudliq,
}

#[derive(Resource)]
pub struct UnitModels {
    pub names_to_handles: HashMap<UnitName, Handle<Scene>>,
}

#[derive(Component)]
pub struct UnitSpawnedMarker;

pub fn spawn_units(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Transform, &Unit), Without<UnitSpawnedMarker>>,
    mut unit_models: ResMut<UnitModels>,
) {
    for (entity, transform, unit) in query.iter() {
        let unit_handle = if let Some(model_handle) =
            unit_models.names_to_handles.get(&unit.unit_name)
        {
            model_handle.clone()
        } else {
            let handle = asset_server.load(unit.model_path);
            unit_models
                .names_to_handles
                .insert(unit.unit_name, handle.clone());
            handle
        };
        commands.entity(entity).insert((
            SceneBundle {
                scene: unit_handle,
                transform: transform.clone().with_translation(
                    transform.translation + unit.spawn_offset,
                ),
                ..Default::default()
            },
            RotateInDirectionOfMovement::default(),
            TiltInDirectionOfMovement::default(),
            StepAnimation::default(),
            GlueToGround::default(),
            SteeringAgent,
            KinematicCharacterControllerBundle::default(),
            ReplaceMaterialKeepTextureMarker {
                material: default_toon_shader_material(),
            },
        ));

        commands.entity(entity).insert(UnitSpawnedMarker);
    }
}

pub fn get_unit_data(name: UnitName) -> Unit {
    match name {
        UnitName::Dudliq => Unit {
            unit_name: UnitName::Dudliq,
            stack_size: 16,
            model_path: "models/dudliq.glb#Scene0",
            spawn_offset: Vec3 {
                x: 0.,
                y: 0.45,
                z: 0.,
            },
        },
        UnitName::None => Unit::default(),
    }
}
