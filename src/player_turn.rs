use crate::player_entity::*;
use bevy::prelude::*;

pub struct PlayerTurn(pub Player);

impl Default for PlayerTurn {
    fn default() -> Self {
        Self(Player::Player1)
    }
}

impl PlayerTurn {
    pub fn change(&mut self) {
        self.0 = match self.0 {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player3,
            Player::Player3 => Player::Player4,
            Player::Player4 => Player::Player1,
        };
    }
}

pub struct PlayerTurnPlugin;
impl Plugin for PlayerTurnPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PlayerTurn>();
    }
}
