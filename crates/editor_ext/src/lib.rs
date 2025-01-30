use bevy::prelude::*;
use bevy_editor_pls::editor_window::{EditorWindow, EditorWindowContext};
use bevy_editor_pls::{egui, prelude::*};

pub struct ComponentSearchWindow;

#[derive(Default)]
pub struct ComponentSearchState {
    search_query: String,
    matching_entities: Vec<Entity>,
    selected_entity: Option<Entity>,
}

impl EditorWindow for ComponentSearchWindow {
    type State = ComponentSearchState;
    const NAME: &'static str = "Component Search";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let state = cx.state_mut::<ComponentSearchWindow>().unwrap();

        // Search input
        ui.horizontal(|ui| {
            ui.label("Search components:");
            if ui.text_edit_singleline(&mut state.search_query).changed() {
                state.matching_entities =
                    find_matching_entities(world, &state.search_query);
            }
        });

        // Display results
        ui.separator();
        ui.horizontal(|ui| {});
        // List of entities
        ui.group(|ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for &entity in &state.matching_entities {
                    if let Some(entity_ref) = world.get_entity(entity) {
                        let name = entity_ref.get::<Name>().map_or_else(
                            || format!("Entity {}", entity.index()),
                            |n| n.to_string(),
                        );

                        let is_selected = state.selected_entity == Some(entity);
                        if ui.selectable_label(is_selected, name).clicked() {
                            state.selected_entity = Some(entity);
                        }
                    }
                }
            });
        });

        // Show components of selected entity
        if let Some(selected) = state.selected_entity {
            if let Some(entity_ref) = world.get_entity(selected) {
                ui.separator();
                ui.heading("Components:");
                for component_id in entity_ref.archetype().components() {
                    if let Some(info) =
                        world.components().get_info(component_id)
                    {
                        ui.label(info.name());
                    }
                }
            }
        }
    }
}

fn find_matching_entities(world: &World, search_query: &str) -> Vec<Entity> {
    if search_query.is_empty() {
        return Vec::new();
    }

    let search_lower = search_query.to_lowercase();
    let mut matching = Vec::new();

    for entity in world.iter_entities() {
        for component_id in entity.archetype().components() {
            if let Some(info) = world.components().get_info(component_id) {
                if info.name().to_lowercase().contains(&search_lower) {
                    matching.push(entity.id());
                    break;
                }
            }
        }
    }

    matching
}
