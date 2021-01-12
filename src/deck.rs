use crate::card::*;

use bevy::prelude::*;

#[derive(Default)]
pub struct DeckCreationEvent();

#[derive(Debug)]
pub struct Deck {
    pub card_count: i32,
    pub x: f32,
    pub z: f32,
}

impl Default for Deck {
    fn default() -> Deck {
        Deck {
            card_count: 0,
            x: 0.,
            z: -10.,
        }
    }
}

fn on_card_creation(query: Query<&Card, Added<Card>>, mut deck: ResMut<Deck>) {
    for _ in query.iter() {
        deck.card_count += 1;
    }
}

fn on_draw(query_cards: Query<&Card, Mutated<Card>>, mut deck: ResMut<Deck>) {
    for card in query_cards.iter() {
        if let CardOwner::Deck = card.previous_owner {
            if deck.card_count > 0 {
                match card.owner {
                    CardOwner::Deck | CardOwner::Discard => (),
                    _ => deck.card_count -= 1,
                }
            }
        }
    }
}

pub struct DeckPlugin;
impl Plugin for DeckPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Deck>()
            .add_system(on_draw.system())
            .add_system(on_card_creation.system());
    }
}
