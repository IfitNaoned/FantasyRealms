use crate::{app_state::*, discard::*, player_entity::*, player_phase::*, player_turn::*, *};
use bevy::prelude::*;
use bevy_mod_picking::*;
use rand::thread_rng;
use rand::Rng;

pub const PLAYER_FIRST_HAND_CARD_COUNT: i32 = 7;
//pub const DECK_CARD_COUNT: i32 = 60;

#[derive(Debug)]
pub struct DrawEvent(pub Player);
impl Default for DrawEvent {
    fn default() -> Self {
        Self(Player::Player1)
    }
}

pub struct DiscardEvent(pub Player);
impl Default for DiscardEvent {
    fn default() -> Self {
        Self(Player::Player1)
    }
}

#[derive(Default)]
struct SelectedCard {
    entity: Option<Entity>,
}

#[derive(Debug, Clone)]
pub enum CardOwner {
    Player1,
    Player2,
    Player3,
    Player4,
    Deck,
    Discard,
}
#[derive(Debug)]
pub enum CardType {
    AirElemental,
    Basilisk,
    BeastMaster,
    BellTower,
    Blizzard,
    BookOfChanges,
    Candle,
    Cavern,
    Collector,
    Doppleganger,
    Dragon,
    DwarvishInfantry,
    EarthElemental,
    ElevenArcher,
    ElvenLongBow,
    Empress,
    Enchantress,
    FireElemental,
    Forest,
    Forge,
    Fountain,
    GemOfOrder,
    GreatFlood,
    Hydra,
    Island,
    Jester,
    King,
    Knights,
    LightCavalry,
    Lightning,
    MagicWand,
    Mirage,
    Mountain,
    Necromancer,
    Princess,
    ProtectionRune,
    Queen,
    RainStorm,
    Rangers,
    Shapeshifter,
    ShieldofKeth,
    Smoke,
    Swamp,
    SwordofKeth,
    Unicorn,
    WarDirigible,
    Warhorse,
    WarlockLord,
    Warlord,
    Warship,
    WaterElemental,
    Whirlwind,
    Wildfire,
    WorldTree,
}

#[derive(Debug)]
pub struct Card {
    pub owner: CardOwner,
    pub previous_owner: CardOwner,
    pub card_type: CardType,
    pub position: i32,
}

impl Card {
    pub fn change_owner(&mut self, new_owner: CardOwner) {
        self.previous_owner = self.owner.clone();
        self.owner = new_owner;
    }
}

fn get_player_from_cardowner(card_owner: CardOwner) -> Option<Player> {
    match card_owner {
        CardOwner::Player1 => return Some(Player::Player1),
        CardOwner::Player2 => return Some(Player::Player2),
        CardOwner::Player3 => return Some(Player::Player3),
        CardOwner::Player4 => return Some(Player::Player4),
        _ => return None,
    }
}

fn create_deck(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    deck: Res<Deck>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut app_state: ResMut<State<AppState>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut all_cards = Vec::new();
    all_cards.push(CardType::AirElemental);
    all_cards.push(CardType::Basilisk);
    all_cards.push(CardType::BeastMaster);
    all_cards.push(CardType::BellTower);
    all_cards.push(CardType::Blizzard);
    all_cards.push(CardType::BookOfChanges);
    all_cards.push(CardType::Candle);
    all_cards.push(CardType::Cavern);
    all_cards.push(CardType::Collector);
    all_cards.push(CardType::Doppleganger);
    all_cards.push(CardType::Dragon);
    all_cards.push(CardType::DwarvishInfantry);
    all_cards.push(CardType::EarthElemental);
    all_cards.push(CardType::ElevenArcher);
    all_cards.push(CardType::ElvenLongBow);
    all_cards.push(CardType::Empress);
    all_cards.push(CardType::Enchantress);
    all_cards.push(CardType::FireElemental);
    all_cards.push(CardType::Forest);
    all_cards.push(CardType::Forge);
    all_cards.push(CardType::Fountain);
    all_cards.push(CardType::GemOfOrder);
    all_cards.push(CardType::GreatFlood);
    all_cards.push(CardType::Hydra);
    all_cards.push(CardType::Island);
    all_cards.push(CardType::Jester);
    all_cards.push(CardType::King);
    all_cards.push(CardType::Knights);
    all_cards.push(CardType::LightCavalry);
    all_cards.push(CardType::Lightning);
    all_cards.push(CardType::MagicWand);
    all_cards.push(CardType::Mirage);
    all_cards.push(CardType::Mountain);
    all_cards.push(CardType::Necromancer);
    all_cards.push(CardType::Princess);
    all_cards.push(CardType::ProtectionRune);
    all_cards.push(CardType::Queen);
    all_cards.push(CardType::RainStorm);
    all_cards.push(CardType::Rangers);
    all_cards.push(CardType::Shapeshifter);
    all_cards.push(CardType::ShieldofKeth);
    all_cards.push(CardType::Smoke);
    all_cards.push(CardType::Swamp);
    all_cards.push(CardType::SwordofKeth);
    all_cards.push(CardType::Unicorn);
    all_cards.push(CardType::WarDirigible);
    all_cards.push(CardType::Warhorse);
    all_cards.push(CardType::WarlockLord);
    all_cards.push(CardType::Warlord);
    all_cards.push(CardType::Warship);
    all_cards.push(CardType::WaterElemental);
    all_cards.push(CardType::Whirlwind);
    all_cards.push(CardType::Wildfire);
    all_cards.push(CardType::WorldTree);

    let back_card_material_handle = materials.add(StandardMaterial {
        albedo: Color::rgb(0.51, 0.25, 0.),
        ..Default::default()
    });
    let mesh = meshes.add(Mesh::from(shape::Plane { size: 2. }));
    let mut rng = thread_rng();
    let mut i = 0;

    while all_cards.len() > 0 {
        let random_value = rng.gen_range(0..all_cards.len());
        let card_type = all_cards.remove(random_value);

        let texture_path = match card_type {
            CardType::AirElemental => "models/cards/Card__AirElemental.png",
            CardType::Basilisk => "models/cards/Card__Basilisk.png",
            CardType::BeastMaster => "models/cards/Card__Beastmaster.png",
            CardType::BellTower => "models/cards/Card__BellTower.png",
            CardType::Blizzard => "models/cards/Card__Blizzard.png",
            CardType::BookOfChanges => "models/cards/Card__BookofChanges.png",
            CardType::Candle => "models/cards/Card__Candle.png",
            CardType::Cavern => "models/cards/Card__Cavern.png",
            CardType::Collector => "models/cards/Card__Collector.png",
            CardType::Doppleganger => "models/cards/Card__Doppleganger.png",
            CardType::Dragon => "models/cards/Card__Dragon.png",
            CardType::DwarvishInfantry => "models/cards/Card__DwarvishInfantry.png",
            CardType::EarthElemental => "models/cards/Card__EarthElemental.png",
            CardType::ElevenArcher => "models/cards/Card__ElvenArchers.png",
            CardType::ElvenLongBow => "models/cards/Card__ElvenLongbow.png",
            CardType::Empress => "models/cards/Card__Empress.png",
            CardType::Enchantress => "models/cards/Card__Enchantress.png",
            CardType::FireElemental => "models/cards/Card__FireElemental.png",
            CardType::Forest => "models/cards/Card__Forest.png",
            CardType::Forge => "models/cards/Card__Forge.png",
            CardType::Fountain => "models/cards/Card__Fountain.png",
            CardType::GemOfOrder => "models/cards/Card__GemofOrder.png",
            CardType::GreatFlood => "models/cards/Card__GreatFlood.png",
            CardType::Hydra => "models/cards/Card__Hydra.png",
            CardType::Island => "models/cards/Card__Island.png",
            CardType::Jester => "models/cards/Card__Jester.png",
            CardType::King => "models/cards/Card__King.png",
            CardType::Knights => "models/cards/Card__Knights.png",
            CardType::LightCavalry => "models/cards/Card__LightCavalry.png",
            CardType::Lightning => "models/cards/Card__Lightning.png",
            CardType::MagicWand => "models/cards/Card__MagicWand.png",
            CardType::Mirage => "models/cards/Card__Mirage.png",
            CardType::Mountain => "models/cards/Card__Mountain.png",
            CardType::Necromancer => "models/cards/Card__Necormancer.png",
            CardType::Princess => "models/cards/Card__Princess.png",
            CardType::ProtectionRune => "models/cards/Card__ProtectionRune.png",
            CardType::Queen => "models/cards/Card__Queen.png",
            CardType::RainStorm => "models/cards/Card__Rainstorm.png",
            CardType::Rangers => "models/cards/Card__Rangers.png",
            CardType::Shapeshifter => "models/cards/Card__Shapeshifter.png",
            CardType::ShieldofKeth => "models/cards/Card__ShieldofKeth.png",
            CardType::Smoke => "models/cards/Card__Smoke.png",
            CardType::Swamp => "models/cards/Card__Swamp.png",
            CardType::SwordofKeth => "models/cards/Card__SwordofKeth.png",
            CardType::Unicorn => "models/cards/Card__Unicorn.png",
            CardType::WarDirigible => "models/cards/Card__WarDirigible.png",
            CardType::Warhorse => "models/cards/Card__Warhorse.png",
            CardType::WarlockLord => "models/cards/Card__WarlockLord.png",
            CardType::Warlord => "models/cards/Card__Warlord.png",
            CardType::Warship => "models/cards/Card__Warship.png",
            CardType::WaterElemental => "models/cards/Card__WaterElemental.png",
            CardType::Whirlwind => "models/cards/Card__Whirlwind.png",
            CardType::Wildfire => "models/cards/Card__Wildfire.png",
            CardType::WorldTree => "models/cards/Card__WorldTree.png",
        };

        let texture_handle = asset_server.load(texture_path);
        let card_material_handle = materials.add(StandardMaterial {
            albedo_texture: Some(texture_handle.clone()),
            shaded: false,
            ..Default::default()
        });

        i += 1;
        create_card(
            commands,
            &asset_server,
            mesh.clone(),
            card_material_handle.clone(),
            back_card_material_handle.clone(),
            card_type,
            i,
            Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(0., -0.5, 0., 0.5).normalize(),
                Vec3::new(deck.x, i as f32 * 0.1, deck.z),
            )),
        );
    }
    app_state.overwrite_next(AppState::Started).unwrap();
}

fn create_card(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    plane_mesh: Handle<Mesh>,
    card_material_handle: Handle<StandardMaterial>,
    back_card_material_handle: Handle<StandardMaterial>,
    card_type: CardType,
    position: i32,
    transform: Transform,
) {
    let card_handle: Handle<Mesh> = asset_server.load("models/card.gltf#Mesh0/Primitive0");

    let card = Card {
        owner: CardOwner::Deck,
        previous_owner: CardOwner::Deck,
        card_type,
        position,
    };

    commands
        .spawn(PbrBundle {
            mesh: card_handle.clone(),
            transform: {
                let mut transform = transform;
                transform.rotate(Quat::from_rotation_y(std::f32::consts::PI));
                transform.rotate(Quat::from_rotation_x(std::f32::consts::PI));
                transform
            },
            material: back_card_material_handle,
            ..Default::default()
        })
        .with(PickableMesh::default())
        .with(card)
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: plane_mesh,
                material: card_material_handle,
                transform: {
                    let mut transform = Transform::from_translation(Vec3::new(0., 0.1, 0.));
                    transform.apply_non_uniform_scale(Vec3::new(1., 1., 1.5));
                    transform
                },
                ..Default::default()
            });
        });
}

fn on_card_owner_changed(
    mut query_cards: Query<(&mut Card, &mut Transform), Mutated<Card>>,
    mut discard: ResMut<Discard>,
    mut player_query: Query<&mut PlayerEntity>,
) {
    for (mut card, mut transform) in query_cards.iter_mut() {
        match card.owner {
            CardOwner::Player1 => {
                if let Some(mut player_entity) = player_query
                    .iter_mut()
                    .filter(|player_entity| {
                        if let Player::Player1 = player_entity.player {
                            return true;
                        }
                        return false;
                    })
                    .next()
                {
                    if let (new_slot_position, Some(slot)) =
                        player_entity.get_free_slot_position_and_transform()
                    {
                        if let Some(mut previous_player_entity) = player_query
                            .iter_mut()
                            .filter(|previous_player_entity| {
                                if let Some(previous_player) =
                                    get_player_from_cardowner(card.previous_owner.clone())
                                {
                                    if previous_player == previous_player_entity.player {
                                        return true;
                                    }
                                }
                                return false;
                            })
                            .next()
                        {
                            previous_player_entity.release_slot(card.position);
                        }
                        card.position = new_slot_position;
                        *transform = slot.clone();
                    }
                }
            }
            CardOwner::Player2 => {
                if let Some(mut player_entity) = player_query
                    .iter_mut()
                    .filter(|player_entity| {
                        if let Player::Player2 = player_entity.player {
                            return true;
                        }
                        return false;
                    })
                    .next()
                {
                    if let (new_slot_position, Some(slot)) =
                        player_entity.get_free_slot_position_and_transform()
                    {
                        if let Some(mut previous_player_entity) = player_query
                            .iter_mut()
                            .filter(|previous_player_entity| {
                                if let Some(previous_player) =
                                    get_player_from_cardowner(card.previous_owner.clone())
                                {
                                    if previous_player == previous_player_entity.player {
                                        return true;
                                    }
                                }
                                return false;
                            })
                            .next()
                        {
                            previous_player_entity.release_slot(card.position);
                        }
                        card.position = new_slot_position;
                        *transform = slot.clone();
                    }
                }
            }
            CardOwner::Player3 => {
                if let Some(mut player_entity) = player_query
                    .iter_mut()
                    .filter(|player_entity| {
                        if let Player::Player3 = player_entity.player {
                            return true;
                        }
                        return false;
                    })
                    .next()
                {
                    if let (new_slot_position, Some(slot)) =
                        player_entity.get_free_slot_position_and_transform()
                    {
                        if let Some(mut previous_player_entity) = player_query
                            .iter_mut()
                            .filter(|previous_player_entity| {
                                if let Some(previous_player) =
                                    get_player_from_cardowner(card.previous_owner.clone())
                                {
                                    if previous_player == previous_player_entity.player {
                                        return true;
                                    }
                                }
                                return false;
                            })
                            .next()
                        {
                            previous_player_entity.release_slot(card.position);
                        }
                        card.position = new_slot_position;
                        *transform = slot.clone();
                    }
                }
            }
            CardOwner::Player4 => {
                if let Some(mut player_entity) = player_query
                    .iter_mut()
                    .filter(|player_entity| {
                        if let Player::Player4 = player_entity.player {
                            return true;
                        }
                        return false;
                    })
                    .next()
                {
                    if let (new_slot_position, Some(slot)) =
                        player_entity.get_free_slot_position_and_transform()
                    {
                        if let Some(mut previous_player_entity) = player_query
                            .iter_mut()
                            .filter(|previous_player_entity| {
                                if let Some(previous_player) =
                                    get_player_from_cardowner(card.previous_owner.clone())
                                {
                                    if previous_player == previous_player_entity.player {
                                        return true;
                                    }
                                }
                                return false;
                            })
                            .next()
                        {
                            previous_player_entity.release_slot(card.position);
                        }
                        card.position = new_slot_position;
                        *transform = slot.clone();
                    }
                }
            }
            CardOwner::Discard => {
                if let (new_slot_position, Some(slot_transform)) =
                    discard.get_free_slot_pos_and_transform()
                {
                    if let Some(mut previous_player_entity) = player_query
                        .iter_mut()
                        .filter(|previous_player_entity| {
                            if let Some(previous_player) =
                                get_player_from_cardowner(card.previous_owner.clone())
                            {
                                if previous_player == previous_player_entity.player {
                                    return true;
                                }
                            }
                            return false;
                        })
                        .next()
                    {
                        previous_player_entity.release_slot(card.position);
                    }
                    card.position = new_slot_position;
                    *transform = slot_transform.clone();
                }
            }
            _ => (),
        }
    }
}

fn on_card_selection(
    pick_state: Res<PickState>,
    mut selected_card: ResMut<SelectedCard>,
    mut card_query: Query<&mut Card>,
    mouse_button_inputs: Res<Input<MouseButton>>,
    player_turn: Res<PlayerTurn>,
    player_phase: Res<PlayerPhase>,
    mut draw_event: ResMut<Events<DrawEvent>>,
    mut discard_event: ResMut<Events<DiscardEvent>>,
) {
    if !mouse_button_inputs.just_pressed(MouseButton::Left) {
        return;
    }

    match player_turn.0 {
        Player::Player1 => (),
        _ => return,
    }

    if let Some((card_entity, _intersection)) = pick_state.top(Group::default()) {
        if let Ok(card) = card_query.get_mut(*card_entity) {
            match player_phase.0 {
                Phase::Draw => match card.owner {
                    CardOwner::Deck => draw_event.send(DrawEvent {
                        0: player_turn.0.clone(),
                    }),
                    CardOwner::Discard => {
                        selected_card.entity = Some(*card_entity);
                        draw_event.send(DrawEvent {
                            0: player_turn.0.clone(),
                        })
                    }
                    _ => (),
                },
                Phase::Discard => match card.owner {
                    CardOwner::Player1 => {
                        selected_card.entity = Some(*card_entity);
                        discard_event.send(DiscardEvent {
                            0: player_turn.0.clone(),
                        })
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    }
}

fn on_draw_event(
    mut selected_card: ResMut<SelectedCard>,
    draw_events: Res<Events<DrawEvent>>,
    mut event_reader: Local<EventReader<DrawEvent>>,
    mut card_query: Query<&mut Card>,
    mut player_phase: ResMut<PlayerPhase>,
    player_turn: ResMut<PlayerTurn>,
    deck: Res<Deck>,
) {
    match player_turn.0 {
        Player::Player1 => {
            for draw_event in event_reader.iter(&draw_events) {
                if let Some(card_entity) = selected_card.entity {
                    //from discard
                    selected_card.entity = None;
                    if let Ok(mut card) = card_query.get_mut(card_entity) {
                        match draw_event.0 {
                            Player::Player1 => {
                                card.change_owner(CardOwner::Player1);
                            }
                            Player::Player2 => {
                                card.change_owner(CardOwner::Player1);
                            }
                            Player::Player3 => {
                                card.change_owner(CardOwner::Player1);
                            }
                            Player::Player4 => {
                                card.change_owner(CardOwner::Player1);
                            }
                        }
                        player_phase.change();
                    }
                } else {
                    //from deck
                    for mut card in card_query.iter_mut() {
                        if let CardOwner::Deck = card.owner {
                            if card.position == deck.card_count {
                                match draw_event.0 {
                                    Player::Player1 => {
                                        card.change_owner(CardOwner::Player1);
                                    }
                                    Player::Player2 => {
                                        card.change_owner(CardOwner::Player2);
                                    }
                                    Player::Player3 => {
                                        card.change_owner(CardOwner::Player3);
                                    }
                                    Player::Player4 => {
                                        card.change_owner(CardOwner::Player4);
                                    }
                                }
                                player_phase.change();
                            }
                        }
                    }
                };
            }
        }
        //IA Draw only form deck for the moment, todo : add draw from discard
        _ => {
            for draw_event in event_reader.iter(&draw_events) {
                for mut card in card_query.iter_mut() {
                    if let CardOwner::Deck = card.owner {
                        if card.position == deck.card_count {
                            match draw_event.0 {
                                Player::Player1 => {
                                    card.change_owner(CardOwner::Player1);
                                }
                                Player::Player2 => {
                                    card.change_owner(CardOwner::Player2);
                                }
                                Player::Player3 => {
                                    card.change_owner(CardOwner::Player3);
                                }
                                Player::Player4 => {
                                    card.change_owner(CardOwner::Player4);
                                }
                            }
                            player_phase.change();
                        }
                    }
                }
            }
        }
    }
}

fn on_discard_event(
    mut selected_card: ResMut<SelectedCard>,
    discard_events: Res<Events<DiscardEvent>>,
    mut event_reader: Local<EventReader<DiscardEvent>>,
    mut card_query: Query<&mut Card>,
    mut player_phase: ResMut<PlayerPhase>,
    mut player_turn: ResMut<PlayerTurn>,
) {
    match player_turn.0 {
        Player::Player1 => {
            for discard_event in event_reader.iter(&discard_events) {
                if let Some(card_entity) = selected_card.entity {
                    selected_card.entity = None;
                    if let Ok(mut card) = card_query.get_mut(card_entity) {
                        match discard_event.0 {
                            Player::Player1 => {
                                if let CardOwner::Player1 = card.owner {
                                    card.change_owner(CardOwner::Discard);
                                }
                            }
                            Player::Player2 => {
                                if let CardOwner::Player2 = card.owner {
                                    card.change_owner(CardOwner::Discard);
                                }
                            }
                            Player::Player3 => {
                                if let CardOwner::Player3 = card.owner {
                                    card.change_owner(CardOwner::Discard);
                                }
                            }
                            Player::Player4 => {
                                if let CardOwner::Player4 = card.owner {
                                    card.change_owner(CardOwner::Discard);
                                }
                            }
                        }
                        player_phase.change();
                        player_turn.change();
                    }
                }
            }
        }
        // IA discard
        _ => {
            for discard_event in event_reader.iter(&discard_events) {
                for mut card in card_query.iter_mut() {
                    match discard_event.0 {
                        Player::Player2 => {
                            if let CardOwner::Player2 = card.owner {
                                card.change_owner(CardOwner::Discard);
                                player_phase.change();
                                player_turn.change();
                                return;
                            }
                        }
                        Player::Player3 => {
                            if let CardOwner::Player3 = card.owner {
                                card.change_owner(CardOwner::Discard);
                                player_phase.change();
                                player_turn.change();
                                return;
                            }
                        }
                        Player::Player4 => {
                            if let CardOwner::Player4 = card.owner {
                                card.change_owner(CardOwner::Discard);
                                player_phase.change();
                                player_turn.change();
                                return;
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

fn deal_cards_to_players(
    mut card_query: Query<&mut Card>,
    mut deck: ResMut<Deck>,
    query_players: Query<&PlayerEntity>,
) {
    let player_card_count = PLAYER_FIRST_HAND_CARD_COUNT;
    for _ in 0..player_card_count {
        for player_entity in query_players.iter() {
            for mut card in card_query.iter_mut() {
                if let CardOwner::Deck = card.owner {
                    if card.position == deck.card_count {
                        match player_entity.player {
                            Player::Player1 => card.change_owner(CardOwner::Player1),
                            Player::Player2 => card.change_owner(CardOwner::Player2),
                            Player::Player3 => card.change_owner(CardOwner::Player3),
                            Player::Player4 => card.change_owner(CardOwner::Player4),
                        }
                        deck.card_count -= 1;
                    }
                }
            }
        }
    }
}

pub struct CardPlugin;
impl Plugin for CardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<SelectedCard>()
            .add_event::<DrawEvent>()
            .add_event::<DiscardEvent>()
            .add_system(on_draw_event.system())
            .add_system(on_discard_event.system())
            .add_system(on_card_selection.system())
            .on_state_update(STAGE, AppState::Started, on_card_owner_changed.system())
            .on_state_enter(STAGE, AppState::Started, deal_cards_to_players.system())
            .add_startup_system(create_deck.system());
    }
}
