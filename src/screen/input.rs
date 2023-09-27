use bevy_trait_query::One;
use valence::prelude::*;
use valence::inventory::{HeldItem, DropItemStackEvent, UpdateSelectedSlotEvent};
use valence::interact_item::InteractItemEvent;
use valence::hand_swing::HandSwingEvent;
use valence::event_loop::PacketEvent;
use valence::movement::MovementEvent;
use valence::protocol::bytes::Buf;
use valence::nbt::compound;
use crate::screen::Screen;

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

    pub fn apply(&self, position: &(i32, i32), times: u32) -> (i32, i32) {
        match self {
            MoveDir::Up => (position.0, position.1 - times as i32),
            MoveDir::Left => (position.0 - times as i32, position.1),
            MoveDir::Down => (position.0, position.1 + times as i32),
            MoveDir::Right => (position.0 + times as i32, position.1),
        }
    }
}

pub enum PlayerAction {
    // Left click
    Primary {
        position : Option<(u32, u32)>,
        is_sneaking : bool,
    },
    // Right click
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
    old_screen_position : Option<(u32, u32)>,
    screen : Entity,
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

pub fn init_client(commands: &mut Commands, data: &mut GameData, client: Entity, screen: Entity) {
    commands.get_entity(client).unwrap().insert(PlayerData {
        uid : next_uid(data),
        is_sneaking : false,
        prevent_primary : false,
        old_slot : 0,
        old_screen_position : None,
        screen,
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
    mut screens: Query<&Screen>,
    mut managers: Query<One<&mut dyn GameManager>>,
    player_datas: Query<&PlayerData>,
    mut clients: RemovedComponents<Client>,
) {

    for client in clients.iter() {
        if let Ok(player_data) = player_datas.get(client) {
            free_uid(data.as_mut(), player_data.uid);
            if let Ok(screen) = screens.get_mut(player_data.screen) {
                if let Ok(mut manager) = managers.get_mut(screen.manager) {
                    manager.action(player_data.uid, PlayerAction::Disconnect);
                }
            }
        }
    }
}

fn project_position(screen: &Screen, position: &Position, look: &Look, is_sneaking: bool) -> Option<(u32, u32)> {
    fn euler_angles_to_vec(yaw: f32, pitch: f32) -> DVec3 {
        let pitch = pitch as f64 / 180.0 * std::f64::consts::PI;
        let yaw = yaw as f64 / 180.0 * std::f64::consts::PI;

        let x = -yaw.sin() * pitch.cos();
        let y = -pitch.sin();
        let z = yaw.cos() * pitch.cos();

        DVec3::new(x, y, z)
    }

    fn line_plane_intersection(ray_direction: DVec3, ray_point: DVec3, plane_normal: DVec3, plane_point: DVec3) -> Option<DVec3> {
        // https://www.rosettacode.org/wiki/Find_the_intersection_of_a_line_with_a_plane#Rust
        let dot = ray_direction.dot(plane_normal);

        if dot > 1e-6 {
            None
        } else {
            let distance = (ray_point - plane_point).dot(plane_normal) / dot;
            Some(ray_point - ray_direction * distance)
        }
    }

    fn move_to_eyes(mut position: DVec3, is_sneaking: bool) -> DVec3 {
        position.y += if is_sneaking { 1.3125 } else { 1.625 };
        position
    }

    let ray_direction = euler_angles_to_vec(look.yaw, look.pitch);
    let ray_point = move_to_eyes(position.0, is_sneaking);

    // Hardcoded since you can't rotate screens atm
    let plane_normal = DVec3::new(0.0, 0.0, 1.0);
    let plane_point = screen.position.0;

    let intersection = line_plane_intersection(ray_direction, ray_point, plane_normal, plane_point)?;
    let diff = intersection - screen.position.0;

    let pixel_offset = 1.0 / (screen.pixel_size as f64 * 2.0);
    let size_width = pixel_offset * screen.width as f64;
    let size_height = pixel_offset * screen.height as f64;
    let x = diff.x / size_width;
    let y = 1.0 - diff.y / size_height;

    if !(0.0..1.0).contains(&x) || !(0.0..1.0).contains(&y) {
        None
    } else {
        let x = (x * screen.width as f64).floor() as u32;
        let y = (y * screen.height as f64).floor() as u32;

        Some((x, y))
    }
}

// Some player inputs.
fn process_actions(
    screens: Query<&Screen>,
    mut managers: Query<One<&mut dyn GameManager>>,
    mut clients: Query<(&mut PlayerData, &mut Inventory, &mut HeldItem, &Position, &Look)>,
    mut sneak_event: EventReader<SneakEvent>,
    mut interact_item_event: EventReader<InteractItemEvent>,
    mut hand_swing_event: EventReader<HandSwingEvent>,
    mut packet_event: EventReader<PacketEvent>,
    mut drop_event: EventReader<DropItemStackEvent>,
    mut change_slot_event: EventReader<UpdateSelectedSlotEvent>,
    mut movement_event: EventReader<MovementEvent>,
) {
    fn is_controller(item: &ItemStack) -> bool {
        matches!(&item.nbt, Some(..)) && item.nbt.as_ref().unwrap().contains_key("CustomControllerKey")
    }

    for event in sneak_event.iter() {
        let (mut data, _inventory, _held_item, _position, _look) = clients.get_mut(event.client).unwrap();
        data.is_sneaking = matches!(event.state, SneakState::Start);
    }

    for event in interact_item_event.iter() {
        let (mut data, inventory, held_item, position, look) = clients.get_mut(event.client).unwrap();
        if !is_controller(inventory.slot(held_item.slot())) {
            continue;
        }
        let Ok(screen) = screens.get(data.screen) else {
            continue;
        };
        let Ok(mut manager) = managers.get_mut(screen.manager) else {
            continue;
        };
        data.prevent_primary = true;
        let position = project_position(screen, position, look, data.is_sneaking);
        manager.action(data.uid, PlayerAction::Secondary { position, is_sneaking : data.is_sneaking });
    }

    for event in hand_swing_event.iter() {
        let (data, inventory, held_item, position, look) = clients.get(event.client).unwrap();
        if data.prevent_primary {
            continue;
        }
        if !is_controller(inventory.slot(held_item.slot())) {
            continue;
        }
        let Ok(screen) = screens.get(data.screen) else {
            continue;
        };
        let Ok(mut manager) = managers.get_mut(screen.manager) else {
            continue;
        };
        let position = project_position(screen, position, look, data.is_sneaking);
        manager.action(data.uid, PlayerAction::Primary { position, is_sneaking : data.is_sneaking });
    }

    for event in packet_event.iter() {
        // I didnt find the event for swapping hands, so heres my solution based on wiki.vg/Protocol
        if event.id != 0x1D || event.data.clone().get_u8() != 6 {
            continue;
        }
        let (data, inventory, held_item, _position, _look) = clients.get(event.client).unwrap();
        if !is_controller(inventory.slot(held_item.slot())) {
            continue;
        }
        let Ok(screen) = screens.get(data.screen) else {
            continue;
        };
        let Ok(mut manager) = managers.get_mut(screen.manager) else {
            continue;
        };
        manager.action(data.uid, PlayerAction::Swap { is_sneaking : data.is_sneaking });
    }

    for event in drop_event.iter() {
        if !is_controller(&event.stack) {
            continue;
        }
        let (mut data, mut inventory, _held_item, _position, _look) = clients.get_mut(event.client).unwrap();
        let Ok(screen) = screens.get(data.screen) else {
            continue;
        };
        let Ok(mut manager) = managers.get_mut(screen.manager) else {
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
        let (mut data, inventory, mut held_item, _position, _look) = clients.get_mut(event.client).unwrap();
        let old_slot = data.old_slot;
        data.old_slot = event.slot as u16;
        if event.slot < 5 {
            continue;
        }
        if !is_controller(inventory.slot(old_slot + 36)) {
            continue;
        }
        let Ok(screen) = screens.get(data.screen) else {
            continue;
        };
        let Ok(mut manager) = managers.get_mut(screen.manager) else {
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

    for event in movement_event.iter() {
        let (mut data, inventory, held_item, position, look) = clients.get_mut(event.client).unwrap();
        let Ok(screen) = screens.get(data.screen) else {
            continue;
        };
        let Ok(mut manager) = managers.get_mut(screen.manager) else {
            continue;
        };
        if !is_controller(inventory.slot(held_item.slot())) {
            continue;
        }

        let position = project_position(screen, position, look, data.is_sneaking);
        if data.old_screen_position == position {
            continue;
        }
        data.old_screen_position = position;
        manager.action(data.uid, PlayerAction::Hover { position, is_sneaking: data.is_sneaking });
    }
}

