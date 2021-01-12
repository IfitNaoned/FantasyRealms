use crate::card::*;
use crate::player_entity::*;
use crate::player_turn::*;

use bevy::prelude::*;

#[derive(Debug)]
pub enum Phase {
    None,
    Draw,
    Discard,
}

pub struct PlayerPhase(pub Phase);

impl Default for PlayerPhase {
    fn default() -> Self {
        Self(Phase::None)
    }
}

impl PlayerPhase {
    pub fn change(&mut self) {
        self.0 = match self.0 {
            Phase::None => Phase::Draw,
            Phase::Draw => Phase::Discard,
            Phase::Discard => Phase::None,
        };
    }
}

fn on_player_turn(
    player_turn: ChangedRes<PlayerTurn>,
    mut player_phase: ResMut<PlayerPhase>,
    mut draw_event: ResMut<Events<DrawEvent>>,
) {
    player_phase.change();
    match player_turn.0 {
        Player::Player1 => {}
        //draw IA
        _ => draw_event.send(DrawEvent {
            0: player_turn.0.clone(),
        }),
    }
}

fn on_player_phase(
    player_phase: ChangedRes<PlayerPhase>,
    player_turn: Res<PlayerTurn>,
    mut discard_event: ResMut<Events<DiscardEvent>>,
) {
    if let Phase::Discard = player_phase.0 {
        match player_turn.0 {
            Player::Player1 => (),
            //IA Discard event
            _ => discard_event.send(DiscardEvent {
                0: player_turn.0.clone(),
            }),
        }
    }
}

pub struct PlayerPhasePlugin;
impl Plugin for PlayerPhasePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PlayerPhase>()
            .add_system(on_player_phase.system())
            .add_system(on_player_turn.system());
    }
}
