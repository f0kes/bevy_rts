use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub struct Team {
    pub id: u32,
    pub color: Color,
    pub name: &'static str,
}

pub const TEAM_PLAYER: Team = Team {
    id: 0,
    color: Color::srgb(0.7, 0.0, 0.0),
    name: "Player",
};
pub const TEAM_GOBLIN: Team = Team {
    id: 1,
    color: Color::srgb(0.0, 0.8, 0.3),
    name: "Goblin",
};
impl PartialEq for Team {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
