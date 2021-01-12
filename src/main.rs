mod board;
mod card;
mod deck;
mod discard;
mod player_entity;
mod player_phase;
mod player_turn;
use bevy::prelude::*;
use bevy_mod_picking::*;
use board::*;
use card::*;
use deck::*;
use discard::*;
use player_entity::*;
use player_phase::*;
use player_turn::*;
mod app_state;
use app_state::*;

fn main() {
    App::build()
        // Set antialiasing to use 4 samples
        .add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            title: "Fantasy Realms v2".to_string(),
            width: 800.,
            height: 600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_resource(State::new(AppState::Setup))
        .add_stage_after(stage::UPDATE, STAGE, StateStage::<AppState>::default())
        .add_plugin(BoardPlugin)
        .add_plugin(PlayerEntityPlugin)
        .add_plugin(CardPlugin)
        .add_plugin(PlayerTurnPlugin)
        .add_plugin(PlayerPhasePlugin)
        .add_plugin(DeckPlugin)
        .add_plugin(DiscardPlugin)
        .add_plugin(PickingPlugin)
        //.add_plugin(DebugPickingPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(commands: &mut Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.2, -0.5, -0.2, 0.5).normalize(),
                Vec3::new(-30.0, 30., 0.),
            )),
            ..Default::default()
        })
        .with(PickSource::default())
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 30.0, 0.0)),
            ..Default::default()
        });
}
