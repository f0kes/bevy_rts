
#[macro_export]
macro_rules! create_meta_component {
    ($name:ident, [$($dependent:ty),* $(,)?]) => {
        use bevy::prelude::*;
        use misc::disabled::{ComponentTogglePlugin, ToggleCommands, Disabled};

        // Create the marker component
        #[derive(Component,Default,Clone)]
        pub struct $name;

        // Create the plugin
        paste::paste! {
            // Create the plugin with concatenated name
            pub struct [<$name Plugin>];

            impl Plugin for [<$name Plugin>] {
                fn build(&self, app: &mut App) {
                    // Add toggle plugins for all dependent components
                    app.add_plugins(ComponentTogglePlugin::<$name>::default());
                    $(
                        app.add_plugins(ComponentTogglePlugin::<$dependent>::default());
                    )*
                    // Add systems for handling the meta-component
                    app.add_systems(Update, (
                        on_enter_system::<$name>,
                        propagate_disabled::<$name>,
                        remove_meta_component::<$name>,
                        assume_meta_component::<$name>,
                    ).chain());
                }
            }
        }

        // System to handle when meta-component is added
        fn on_enter_system<T: Component>(
            mut commands: Commands,
            query: Query<Entity, Added<T>>,
        ) {
            for entity in query.iter() {
                let mut entity_commands = commands.entity(entity);
                //println!("Enabling meta component for entity {:?}", entity);
                $(
                    entity_commands.enable::<$dependent>();
                )*
            }
        }

        fn propagate_disabled<T: Component>(
            mut commands: Commands,
            query: Query<Entity, Added<Disabled<T>>>,
        ) {
            for entity in query.iter() {

                let mut entity_commands = commands.entity(entity);
                $(
                    entity_commands.disable::<$dependent>();
                )*

            }
        }

        fn assume_meta_component<T: Component + Default>(
            mut commands: Commands,
            mut query: Query<Entity, ($(With<$dependent>,)* Without<T>, Without<Disabled<T>>)>,
        ) {
            for entity in query.iter_mut() {
                //println!("Assuming meta component for entity {:?}", entity);
                commands.entity(entity).insert(T::default());
            }
        }
        fn remove_meta_component<T: Component>(
            mut commands: Commands,
            mut query: Query<Entity, (Or<($(Without<$dependent>,)*)>, With<T>)>,
        ) {
            for entity in query.iter_mut() {
                commands.entity(entity).remove::<T>();
            }
        }
    };
}
