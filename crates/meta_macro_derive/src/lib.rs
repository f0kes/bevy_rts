// Macro to create meta-components and their plugins
// meta_component_derive/lib.rs
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Type};

#[proc_macro_attribute]
pub fn meta_component(attr: TokenStream, input: TokenStream) -> TokenStream {
    let meta_name = parse_macro_input!(attr as syn::Ident);
    let input = parse_macro_input!(input as DeriveInput);

    let blueprint_name = &input.ident;
    let vis = &input.vis;
    let import = quote! {
        use bevy::prelude::*;
        use misc::disabled::{ComponentTogglePlugin, ToggleCommands, Disabled};
        use bevy::ecs::query::*;
    };
    let fields = match &input.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    // Collect field information
    let field_names: Vec<_> = fields.iter().map(|f| &f.ident).collect();
    let field_types: Vec<_> = fields.iter().map(|f| &f.ty).collect();

    // Generate marker component
    let marker_component = quote! {
        #[derive(Component, Default, Clone)]
        #vis struct #meta_name;
    };

    // Generate plugin
    let plugin_name = format_ident!("{}Plugin", meta_name);
    let plugin = quote! {
        pub struct #plugin_name;

        impl Plugin for #plugin_name {
            fn build(&self, app: &mut App) {
                app.add_plugins(ComponentTogglePlugin::<#meta_name>::default());
                #(
                    app.add_plugins(ComponentTogglePlugin::<#field_types>::default());
                )*

                app.add_systems(Update, (
                    on_enter_system::<#meta_name>,
                    propagate_disabled::<#meta_name>,
                    remove_meta_component::<#meta_name>,
                    assume_meta_component::<#meta_name>,
                ).chain());
            }
        }
    };

    // Generate systems
    let systems = quote! {
        fn on_enter_system<T: Component>(
            mut commands: Commands,
            query: Query<Entity, Added<T>>,
        ) {
            for entity in query.iter() {
                let mut entity_commands = commands.entity(entity);
                #(
                    entity_commands.enable::<#field_types>();
                )*
            }
        }

        fn propagate_disabled<T: Component>(
            mut commands: Commands,
            query: Query<Entity, Added<Disabled<T>>>,
        ) {
            for entity in query.iter() {
                let mut entity_commands = commands.entity(entity);
                #(
                    entity_commands.disable::<#field_types>();
                )*
            }
        }

        fn assume_meta_component<T: Component + Default>(
            mut commands: Commands,
            mut query: Query<Entity, (
                #(With<#field_types>,)*
                Without<T>,
                Without<Disabled<T>>
            )>,
        ) {
            for entity in query.iter_mut() {
                commands.entity(entity).insert(T::default());
            }
        }

        fn remove_meta_component<T: Component>(
            mut commands: Commands,
            mut query: Query<Entity, (Or<(#(Without<#field_types>,)*)>, With<T>)>,
        ) {
            for entity in query.iter_mut() {
                commands.entity(entity).remove::<T>();
            }
        }
    };

    // Generate WorldQuery
    let query_name = format_ident!("{}Query", meta_name);
    let world_query = quote! {
        #[derive(QueryData)]
        pub struct #query_name<'w> {
            #(
                pub #field_names: &'w #field_types,
            )*
        }
    };

    // Implement Default for blueprint
    let default_impl = quote! {
        impl Default for #blueprint_name {
            fn default() -> Self {
                Self {
                    #(
                        #field_names: Default::default(),
                    )*
                }
            }
        }
    };

    let expanded = quote! {
        #import
        #input

        #marker_component
        #plugin
        #systems
        #world_query
        
    };

    expanded.into()
}
