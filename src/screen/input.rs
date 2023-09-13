use bevy_trait_query::One;
use valence::prelude::*;
use valence::inventory::{HeldItem, DropItemStackEvent, UpdateSelectedSlotEvent};
use valence::interact_item::InteractItemEvent;
use valence::hand_swing::HandSwingEvent;
use valence::event_loop::PacketEvent;
use valence::protocol::bytes::Buf;
use valence::nbt::compound;

use super::game_manager::GameManager;

pub type Uid = u8;

#[derive(PartialEq)]
pub enum MoveDir {
    Up,
    Left,
    Down,
    Right,
}

impl MoveDir {
    pub fn opposite(&self) -> MoveDir {
        match self {
            MoveDir::Up => MoveDir::Down,
            MoveDir::Left => MoveDir::Right,
            MoveDir::Down => MoveDir::Up,
            MoveDir::Right => MoveDir::Left,
        }
    }
}

pub enum PlayerAction {
    // Right click
    // TODO: include position on screen
    Primary {
        position : Option<(u32, u32)>,
        is_sneaking : bool,
    },
    // Left click
    // TODO: include position on screen
    Secondary {
        position : Option<(u32, u32)>,
        is_sneaking : bool,
    },
    // Swap Hands
    Swap {
        is_sneaking : bool,
    },
    // Drop
    Drop {
        is_sneaking : bool,
    },
    // Player movement with WASD (requires sitting on something)
    // TODO: implement
    Move {
        direction : MoveDir,
        is_sneaking : bool,
    },
    // Slot movement (last 4 slots of the hotbar)
    SpecialMove {
        direction : MoveDir,
        is_sneaking : bool,
    },
    // Text input via chat/command
    // TODO: implement
    Input {
        input : String,
    },
    // Change of cursor position on screen
    // TODO: implement
    Hover {
        position : Option<(u32, u32)>,
        is_sneaking : bool,
    },
    // Player disconnected and uid is freed
    Disconnect,
}

// Player data to manage inputs
#[derive(Component)]
pub struct PlayerData {
    uid : Uid,
    is_sneaking : bool,
    prevent_primary : bool,
    old_slot : u16,
    manager_id : Entity,
}

// Game data for id management
#[derive(Resource)]
pub struct GameData {
    occupied_uids : [bool; Uid::MAX as usize],
    next_free_uid : Uid,
}

fn next_uid(data: &mut GameData) -> Uid {
    let uid = data.next_free_uid;
    data.occupied_uids[data.next_free_uid as usize] = true;
    while data.occupied_uids[data.next_free_uid as usize] {
        data.next_free_uid += 1;
    }
    uid
}

fn free_uid(data: &mut GameData, uid: Uid) {
    data.occupied_uids[uid as usize] = false;
    if uid < data.next_free_uid {
        data.next_free_uid = uid;
    }
}

pub fn init_client(commands: &mut Commands, data: &mut GameData, client: Entity, manager_id: Entity) {
    commands.get_entity(client).unwrap().insert(PlayerData {
        uid : next_uid(data),
        is_sneaking : false,
        prevent_primary : false,
        old_slot : 0,
        manager_id,
    });
}

pub fn get_controller_item() -> ItemStack {
    ItemStack::new(
        ItemKind::IronHorseArmor,
        1,
        Some(compound! {
            "display" => compound! {
                "Name" => Text::text("Controller").not_italic(),
            },
            // Custom tag for controller
            "CustomControllerKey" => compound! { }
        })
    )
}

pub fn build(app: &mut App) {
    app
        .add_systems(Update, (update_primary_prevention, remove_clients))
        .add_systems(EventLoopUpdate, process_actions)
        .insert_resource(GameData {
            occupied_uids : [false; Uid::MAX as usize],
            next_free_uid : 0,
        });
}

fn update_primary_prevention(mut player_datas: Query<&mut PlayerData>) {
    for mut player_data in player_datas.iter_mut() {
        player_data.prevent_primary = false;
    }
}

pub fn remove_clients(
    mut data: ResMut<GameData>,
    mut managers: Query<One<&mut dyn GameManager>>,
    player_datas: Query<&PlayerData>,
    mut clients: RemovedComponents<Client>,
) {
    for client in clients.iter() {
        if let Ok(player_data) = player_datas.get(client) {
            free_uid(data.as_mut(), player_data.uid);
            if let Ok(mut manager) = managers.get_mut(player_data.manager_id) {
                manager.action(player_data.uid, PlayerAction::Disconnect);
            }
        }
    }
}

// Some player inputs.
fn process_actions(
    mut managers: Query<One<&mut dyn GameManager>>,
    mut clients: Query<(&mut PlayerData, &mut Inventory, &mut HeldItem)>,
    mut sneak_event: EventReader<SneakEvent>,
    mut interact_item_event: EventReader<InteractItemEvent>,
    mut hand_swing_event: EventReader<HandSwingEvent>,
    mut packet_event: EventReader<PacketEvent>,
    mut drop_event: EventReader<DropItemStackEvent>,
    mut change_slot_event: EventReader<UpdateSelectedSlotEvent>,
) {
    fn is_controller(item: &ItemStack) -> bool {
        matches!(&item.nbt, Some(..)) && item.nbt.as_ref().unwrap().contains_key("CustomControllerKey")
    }

    for event in sneak_event.iter() {
        let (mut data, _inventory, _held_item) = clients.get_mut(event.client).unwrap();
        data.is_sneaking = matches!(event.state, SneakState::Start);
    }

    for event in interact_item_event.iter() {
        let (mut data, inventory, held_item) = clients.get_mut(event.client).unwrap();
        if !is_controller(inventory.slot(held_item.slot())) {
            continue;
        }
        let Ok(mut manager) = managers.get_mut(data.manager_id) else {
            continue;
        };
        data.prevent_primary = true;
        manager.action(data.uid, PlayerAction::Secondary { position : None, is_sneaking : data.is_sneaking });
    }

    for event in hand_swing_event.iter() {
        let (data, inventory, held_item) = clients.get(event.client).unwrap();
        if data.prevent_primary {
            continue;
        }
        if !is_controller(inventory.slot(held_item.slot())) {
            continue;
        }
        let Ok(mut manager) = managers.get_mut(data.manager_id) else {
            continue;
        };
        manager.action(data.uid, PlayerAction::Primary { position : None, is_sneaking : data.is_sneaking });
    }

    for event in packet_event.iter() {
        // I didnt find the event for swapping hands, so heres my solution based on wiki.vg/Protocol
        if event.id != 0x1D || event.data.clone().get_u8() != 6 {
            continue;
        }
        let (data, inventory, held_item) = clients.get(event.client).unwrap();
        if !is_controller(inventory.slot(held_item.slot())) {
            continue;
        }
        let Ok(mut manager) = managers.get_mut(data.manager_id) else {
            continue;
        };
        manager.action(data.uid, PlayerAction::Swap { is_sneaking : data.is_sneaking });
    }
    
    for event in drop_event.iter() {
        if !is_controller(&event.stack) {
            continue;
        }
        let (mut data, mut inventory, _held_item) = clients.get_mut(event.client).unwrap();
        let Ok(mut manager) = managers.get_mut(data.manager_id) else {
            continue;
        };
        data.prevent_primary = true;

        manager.action(data.uid, PlayerAction::Drop { is_sneaking : data.is_sneaking });

        // Return item back
        let mut slot = 36;
        if let Some(event_slot) = event.from_slot {
            slot = event_slot;
        } else if let Some(first_slot) = inventory.first_empty_slot_in(9..45) {
            // prioritize hotbar
            slot = if first_slot < 18 { first_slot + 27 } else { first_slot - 9 }
        }
        inventory.set_slot(slot, event.stack.clone());
    }

    for event in change_slot_event.iter() {
        // Cool (imo) movement input based on hotbar slots (last 4 of them)
        let (mut data, inventory, mut held_item) = clients.get_mut(event.client).unwrap();
        let old_slot = data.old_slot;
        data.old_slot = event.slot as u16;
        if event.slot < 5 {
            continue;
        }
        if !is_controller(inventory.slot(old_slot + 36)) {
            continue;
        }
        let Ok(mut manager) = managers.get_mut(data.manager_id) else {
            continue;
        };
        let dir = match event.slot {
            5 => MoveDir::Left,
            6 => MoveDir::Up,
            7 => MoveDir::Down,
            8 => MoveDir::Right,
            _ => unreachable!()
        };
        data.old_slot = old_slot;
        held_item.set_slot(36 + old_slot);
        manager.action(data.uid, PlayerAction::SpecialMove { direction : dir, is_sneaking : data.is_sneaking });
    }
}
