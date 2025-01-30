use bevy::prelude::*;

#[derive(Component)]
pub struct Temporary<T: Component + Clone> {
    pub value: T,
    pub time_left: f32,
}
pub fn apply_temporary<T: Component + Clone>(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Temporary<T>)>,
) {
    for (entity, mut temporary) in query.iter_mut() {
        commands.entity(entity).insert(temporary.value.clone());
    }
}
pub fn update_temporary<T: Component + Clone>(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Temporary<T>)>,
    time: Res<Time>,
) {
    for (entity, mut temporary) in query.iter_mut() {
        temporary.time_left -= time.delta_seconds();
        if temporary.time_left <= 0.0 {
            commands.entity(entity).remove::<Temporary<T>>();
            commands.entity(entity).remove::<T>();
        }
    }
}

pub trait TemporaryAppExt {
    fn add_temporary<T: Component + Clone>(&mut self) -> &mut Self;
}
impl TemporaryAppExt for App {
    fn add_temporary<T: Component + Clone>(&mut self) -> &mut Self {
        self.add_systems(Update, (apply_temporary::<T>, update_temporary::<T>))
    }
}
