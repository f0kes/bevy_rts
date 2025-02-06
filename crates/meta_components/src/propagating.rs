use bevy::prelude::*;

pub trait PropagateAppExt {
    fn propagate<T: Component + Clone + 'static>(&mut self) -> &mut Self;
}
impl PropagateAppExt for App {
    fn propagate<T: Component + Clone + 'static>(&mut self) -> &mut Self {
        self.add_systems(Update, propagate_on_component_change::<T>);
        self.add_systems(Update, propagate_on_hierarchy_change::<T>);
        self
    }
}

pub enum Direction {
    Up,
    Down,
    Both,
}

#[derive(Component)]
pub struct Propagating<T: Component + Clone> {
    pub direction: Direction,
    pub _marker: std::marker::PhantomData<T>,
}

impl<T: Component + Clone> Propagating<T> {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            _marker: std::marker::PhantomData,
        }
    }
    pub fn up() -> Self {
        Self::new(Direction::Up)
    }
    pub fn down() -> Self {
        Self::new(Direction::Down)
    }
    pub fn both() -> Self {
        Self::new(Direction::Both)
    }
}

// Propagate when component T changes
pub fn propagate_on_component_change<T: Component + Clone>(
    query: Query<(Entity, &T, &Propagating<T>), Changed<T>>,
    children_query: Query<&Children>,
    parent_query: Query<&Parent>,
    mut commands: Commands,
) {
    for (entity, component, propagating) in query.iter() {
        println!("Propagating component change");
        match propagating.direction {
            Direction::Up => {
                propagate_component_up(
                    &mut commands,
                    &parent_query,
                    entity,
                    component.clone(),
                );
            }
            Direction::Down => {
                propagate_component_down(
                    &mut commands,
                    &children_query,
                    entity,
                    component.clone(),
                );
            }
            Direction::Both => {
                propagate_component_up(
                    &mut commands,
                    &parent_query,
                    entity,
                    component.clone(),
                );
                propagate_component_down(
                    &mut commands,
                    &children_query,
                    entity,
                    component.clone(),
                );
            }
        }
    }
}

// Propagate when new children/parents are added
pub fn propagate_on_hierarchy_change<T: Component + Clone>(
    query: Query<(Entity, &T, &Propagating<T>)>,
    changed_children: Query<&Children, Changed<Children>>,
    all_children: Query<&Children>,
    changed_parents: Query<&Parent, Changed<Parent>>,
    all_parents: Query<&Parent>,
    mut commands: Commands,
) {
    // Check for new children
    for children in changed_children.iter() {
        for &child in children.iter() {
            if let Ok((entity, component, propagating)) = query.get(child) {
                if matches!(
                    propagating.direction,
                    Direction::Up | Direction::Both
                ) {
                    propagate_component_up(
                        &mut commands,
                        &all_parents,
                        entity,
                        component.clone(),
                    );
                }
            }
        }
    }

    // Check for new parents
    for parent in changed_parents.iter() {
        let parent_entity = parent.get();
        if let Ok((entity, component, propagating)) = query.get(parent_entity) {
            if matches!(
                propagating.direction,
                Direction::Down | Direction::Both
            ) {
                propagate_component_down(
                    &mut commands,
                    &all_children,
                    entity,
                    component.clone(),
                );
            }
        }
    }
}

pub fn get_ancestors_of_entity(
    child: Entity,
    query: &Query<&Parent>,
) -> Vec<Entity> {
    let mut ancestors = Vec::new();
    let mut current_entity = child;
    while let Ok(parent) = query.get(current_entity) {
        ancestors.push(parent.get());
        current_entity = parent.get();
    }
    ancestors
}
pub fn get_descendants_of_entity(
    parent: Entity,
    query: &Query<&Children>,
) -> Vec<Entity> {
    let mut descendants = Vec::new();
    let mut children = Vec::new();
    let mut current_entity = parent;

    while let Ok(children_component) = query.get(current_entity) {
        for child in children_component.iter() {
            children.push(*child);
        }
        if let Some(child) = children.pop() {
            descendants.push(child);
            current_entity = child;
        }
    }

    descendants
}
pub fn propagate_component_down<T: Bundle + Clone>(
    commands: &mut Commands,
    children_query: &Query<&Children>,
    parent: Entity,
    component: T,
) {
    //commands.entity(parent).insert(component.clone());
    let descendants = get_descendants_of_entity(parent, &children_query);
    for descendant in descendants {
        commands.entity(descendant).insert(component.clone());
    }
}
pub fn propagate_component_up<T: Bundle + Clone>(
    commands: &mut Commands,
    parent_query: &Query<&Parent>,
    child: Entity,
    component: T,
) {
    //commands.entity(child).insert(component.clone());
    let mut current_entity = child;
    while let Ok(parent) = parent_query.get(current_entity) {
        let parent_entity = parent.get();
        commands.entity(parent_entity).insert(component.clone());
        current_entity = parent_entity;
    }
}
