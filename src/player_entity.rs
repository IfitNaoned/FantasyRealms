use bevy::prelude::*;
use std::collections::HashMap;

pub const PLAYER_COUNT: i32 = 4;
pub const PLAYER_HAND_SLOT_COUNT: i32 = 8;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Player {
    Player1,
    Player2,
    Player3,
    Player4,
}

#[derive(Debug)]
pub struct PlayerEntity {
    pub player: Player,
    pub slots: HashMap<i32, (bool, Transform)>,
    pub card_count: i32,
}

impl PlayerEntity {
    pub fn get_free_slot_position_and_transform(&mut self) -> (i32, Option<Transform>) {
        for mut used_slot in &mut self.slots {
            if !used_slot.1 .0 {
                used_slot.1 .0 = true;
                return (used_slot.0.clone(), Some(used_slot.1 .1.clone()));
            }
        }
        (0, None)
    }

    pub fn release_slot(&mut self, slot_index: i32) {
        if let Some(mut slot) = self.slots.get_mut(&slot_index) {
            slot.0 = false;
        }
    }
}

fn get_player_by_index(index: i32) -> Option<Player> {
    match index {
        0 => return Some(Player::Player1),
        1 => return Some(Player::Player2),
        2 => return Some(Player::Player3),
        3 => return Some(Player::Player4),
        _ => return None,
    }
}

fn create_players(commands: &mut Commands) {
    for i in 0..PLAYER_COUNT {
        if let Some(player) = get_player_by_index(i) {
            let player_entity = (PlayerEntity {
                card_count: 0,
                slots: {
                    let mut hashmap = HashMap::new();
                    let mut x: f32;
                    let mut y: f32;
                    let mut z: f32;
                    let mut angle_x: f32;
                    let mut angle_y: f32 = 0.;
                    let mut angle_z: f32 = 0.;

                    for i in 0..PLAYER_HAND_SLOT_COUNT {
                        match player {
                            Player::Player1 => {
                                x = -20.;
                                y = 15.;
                                z = (-(i * 2) + PLAYER_HAND_SLOT_COUNT - 1) as f32;
                                angle_x = std::f32::consts::PI + std::f32::consts::PI / 2.;
                                angle_z = std::f32::consts::PI / 2.;
                                angle_y = std::f32::consts::PI / 4.;
                            }
                            Player::Player2 => {
                                x = (-(i * 2) + PLAYER_HAND_SLOT_COUNT - 1) as f32;
                                y = 5.;
                                z = -15.;
                                angle_x = -std::f32::consts::PI / 2.;
                            }
                            Player::Player3 => {
                                x = 15.;
                                y = 5.;
                                z = (-(i * 2) + PLAYER_HAND_SLOT_COUNT - 1) as f32;
                                angle_x = -std::f32::consts::PI / 2.;
                                angle_z = -std::f32::consts::PI / 2.;
                            }
                            Player::Player4 => {
                                x = (-(i * 2) + PLAYER_HAND_SLOT_COUNT - 1) as f32;
                                y = 5.;
                                z = 15.;
                                angle_x = std::f32::consts::PI / 2.;
                            }
                        }

                        let mut transform = Transform::from_translation(Vec3::new(x, y, z));
                        transform.rotate(Quat::from_rotation_x(angle_x));
                        transform.rotate(Quat::from_rotation_y(angle_y));
                        transform.rotate(Quat::from_rotation_z(angle_z));

                        hashmap.insert(i, (false, transform));
                    }
                    hashmap
                },
                player,
            },);
            commands.spawn(player_entity);
        }
    }
}

pub struct PlayerEntityPlugin;
impl Plugin for PlayerEntityPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(create_players.system());
    }
}
