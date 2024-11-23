use bevy::{prelude::*, utils::HashMap};

use crate::action::Action;

#[derive(Default, Resource)]
pub struct InputMap {
    pub map: HashMap<KeyCode, Action>,
}

impl InputMap {
    pub fn bind(&mut self, input: KeyCode, action: Action) {
        self.map.insert(input, action);
    }

    pub fn unbind(&mut self, input: KeyCode) {
        self.map.remove(&input);
    }

    pub fn get(&self, input: KeyCode) -> Option<&Action> {
        self.map.get(&input)
    }

    pub fn get_mut(&mut self, input: KeyCode) -> Option<&mut Action> {
        self.map.get_mut(&input)
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
    pub fn wasd() -> Self {
        InputMap {
            map: HashMap::from([
                (KeyCode::KeyW, Action::MoveForward),
                (KeyCode::KeyA, Action::MoveLeft),
                (KeyCode::KeyS, Action::MoveBack),
                (KeyCode::KeyD, Action::MoveRight),
            ]),
        }
    }
}
pub fn remap_input(
    input_map: Res<InputMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut action_input: ResMut<ButtonInput<Action>>,
) {
    for (key, action) in input_map.map.iter() {
        if keyboard_input.pressed(*key) {
            action_input.press(*action);
        } else if !keyboard_input.pressed(*key) {
            action_input.release(*action);
        }
    }
}
