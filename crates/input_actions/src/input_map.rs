use bevy::{prelude::*, utils::HashMap};

use crate::{action::Action, input_types::InputType};

#[derive(Default, Resource)]
pub struct InputMap {
    pub map: HashMap<InputType, Action>,
}

impl InputMap {
    pub fn bind(&mut self, input: InputType, action: Action) {
        self.map.insert(input, action);
    }

    pub fn unbind(&mut self, input: InputType) {
        self.map.remove(&input);
    }

    pub fn get(&self, input: InputType) -> Option<&Action> {
        self.map.get(&input)
    }

    pub fn get_mut(&mut self, input: InputType) -> Option<&mut Action> {
        self.map.get_mut(&input)
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }
    pub fn wasd() -> Self {
        InputMap {
            map: HashMap::from([
                (KeyCode::KeyW.into(), Action::MoveForward),
                (KeyCode::KeyA.into(), Action::MoveLeft),
                (KeyCode::KeyS.into(), Action::MoveBack),
                (KeyCode::KeyD.into(), Action::MoveRight),
                (MouseButton::Right.into(), Action::Collect),
            ]),
        }
    }
}
pub fn remap_input(
    input_map: Res<InputMap>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut action_input: ResMut<ButtonInput<Action>>,
) {
    for (input, action) in input_map.map.iter() {
        match input {
            InputType::Key(key) => {
                if keyboard_input.pressed(*key) {
                    action_input.press(*action);
                } else {
                    action_input.release(*action);
                }
            }
            InputType::MouseButton(button) => {
                if mouse_input.pressed(*button) {
                    action_input.press(*action);
                } else {
                    action_input.release(*action);
                }
            }
        }
    }
}
