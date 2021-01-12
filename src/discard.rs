use crate::card::*;
use bevy::{app::AppExit, prelude::*};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Discard {
    pub slots: HashMap<i32, (bool, Transform)>,
    pub card_count: i32,
}

impl Default for Discard {
    fn default() -> Discard {
        Discard {
            slots: {
                let mut hashmap = HashMap::new();
                hashmap.insert(
                    1,
                    (false, {
                        let mut transform = Transform::from_translation(Vec3::new(3., 0.1, -3.));
                        transform.rotate(Quat::from_rotation_y(std::f32::consts::PI / 2.));
                        transform
                    }),
                );
                hashmap.insert(
                    2,
                    (false, {
                        let mut transform = Transform::from_translation(Vec3::new(3., 0.1, -0.));
                        transform.rotate(Quat::from_rotation_y(std::f32::consts::PI / 2.));
                        transform
                    }),
                );
                hashmap.insert(
                    3,
                    (false, {
                        let mut transform = Transform::from_translation(Vec3::new(3., 0.1, 3.));
                        transform.rotate(Quat::from_rotation_y(std::f32::consts::PI / 2.));
                        transform
                    }),
                );
                hashmap.insert(
                    4,
                    (false, {
                        let mut transform = Transform::from_translation(Vec3::new(3., 0.1, 6.));
                        transform.rotate(Quat::from_rotation_y(std::f32::consts::PI / 2.));
                        transform
                    }),
                );
                hashmap.insert(
                    5,
                    (false, {
                        let mut transform = Transform::from_translation(Vec3::new(3., 0.1, 9.));
                        transform.rotate(Quat::from_rotation_y(std::f32::consts::PI / 2.));
                        transform
                    }),
                );
                hashmap.insert(
                    6,
                    (false, {
                        let mut transform = Transform::from_translation(Vec3::new(-1., 0.1, -3.));
                        transform.rotate(Quat::from_rotation_y(std::f32::consts::PI / 2.));
                        transform
                    }),
                );
                hashmap.insert(
                    7,
                    (false, {
                        let mut transform = Transform::from_translation(Vec3::new(-1., 0.1, 0.));
                        transform.rotate(Quat::from_rotation_y(std::f32::consts::PI / 2.));
                        transform
                    }),
                );
                hashmap.insert(
                    8,
                    (false, {
                        let mut transform = Transform::from_translation(Vec3::new(-1., 0.1, 3.));
                        transform.rotate(Quat::from_rotation_y(std::f32::consts::PI / 2.));
                        transform
                    }),
                );
                hashmap.insert(
                    9,
                    (false, {
                        let mut transform = Transform::from_translation(Vec3::new(-1., 0.1, 6.));
                        transform.rotate(Quat::from_rotation_y(std::f32::consts::PI / 2.));
                        transform
                    }),
                );
                hashmap.insert(
                    10,
                    (false, {
                        let mut transform = Transform::from_translation(Vec3::new(-1., 0.1, 9.));
                        transform.rotate(Quat::from_rotation_y(std::f32::consts::PI / 2.));
                        transform
                    }),
                );
                hashmap
            },
            card_count: 0,
        }
    }
}

impl Discard {
    pub fn get_free_slot_pos_and_transform(&mut self) -> (i32, Option<Transform>) {
        for mut used_slot in &mut self.slots {
            if !used_slot.1 .0 {
                used_slot.1 .0 = true;
                return (used_slot.0.clone(), Some(used_slot.1 .1.clone()));
            }
        }
        (0, None)
    }
}

fn on_discard(
    mut query_cards: Query<&mut Card, Mutated<Card>>,
    mut discard: ResMut<Discard>,
    mut app_exit_events: ResMut<Events<AppExit>>,
) {
    for card in query_cards.iter_mut() {
        if let CardOwner::Discard = card.owner {
            match card.previous_owner {
                CardOwner::Player1
                | CardOwner::Player2
                | CardOwner::Player3
                | CardOwner::Player4 => {
                    discard.card_count += 1;

                    if discard.card_count >= 10 {
                        //TODO: add score computation + stage
                        app_exit_events.send(AppExit);
                    }
                }
                _ => (),
            }
        }
    }
}

pub struct DiscardPlugin;
impl Plugin for DiscardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Discard>()
            .add_system(on_discard.system());
    }
}
