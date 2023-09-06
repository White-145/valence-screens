use bevy_trait_query::RegisterExt;
use valence::entity::text_display::TextDisplayEntityBundle;
use valence::event_loop::PacketEvent;
use valence::hand_swing::HandSwingEvent;
use valence::interact_item::InteractItemEvent;
use valence::inventory::{DropItemStackEvent, UpdateSelectedSlotEvent, HeldItem};
use valence::prelude::*;
use valence::protocol::bytes::Buf;
use valence::text::color::RgbColor;
use crate::buffer::ScreenBuffer;
use crate::game_manager::matrix_generator::MatrixGenerator;
use crate::game_manager::{GameManager, PlayerAction, MoveDir};
use crate::game_manager::default_game_manager::StaticGameManager;
use crate::game_manager::rainbow_generator::RainbowGenerator;

use bevy_trait_query::One;
use valence::Server;

const BG_PIXEL: char = '■';
const BIAS_PIXEL: char = '█';

#[derive(Clone)]
enum Ground {
    BackGround,
    ForeGround,
}

// Component that is used to store information about screen part
#[derive(Component)]
struct ScreenPart {
    pub screen_height : u32,
    pub x : i32,
    pub i : i32,
    pub previous_result : Text,
    pub ground : Ground,
}

// Foreground style
#[derive(Clone, Copy)]
pub struct Style {
    bold : bool,
    strikethrough : bool,
    underlined : bool,
    italic : bool,
}

impl Default for Style {
    fn default() -> Self {
        Style { bold : false, strikethrough : false, underlined : false, italic : false }
    }
}

impl Style {
    pub fn new(bold: bool, strikethrough: bool, underlined: bool, italic: bool) -> Self {
        Style { bold, strikethrough, underlined, italic }
    }

    pub fn resolve(&self, mut text: Text) -> Text {
        text = if self.bold { text.bold() } else { text.not_bold() };
        text = if self.strikethrough { text.strikethrough() } else { text.not_strikethrough() };
        text = if self.underlined { text.underlined() } else { text.not_underlined() };
        text = if self.italic { text.italic() } else { text.not_italic() };
        text
    }
}

#[derive(Clone)]
pub struct ScreenPixel {
    pub bg : RgbColor,
    pub fg : (char, RgbColor, Style),
}

impl Default for ScreenPixel {
    fn default() -> Self {
        ScreenPixel { bg: RgbColor::new(0, 0, 0), fg: (' ', RgbColor::new(255, 255, 255), Style::default()) }
    }
}

impl ScreenPixel {
    pub fn new(bg: RgbColor, fg_char: char, fg_color: RgbColor, fg_style: Style) -> Self {
        ScreenPixel { bg, fg : (fg_char, fg_color, fg_style) }
    }

    pub fn new_bg(bg: RgbColor) -> Self {
        ScreenPixel { bg, fg : (' ', RgbColor::new(0, 0, 0), Style::default()) }
    }

    pub fn new_fg(fg_char: char, fg_color: RgbColor, fg_style: Style) -> Self {
        ScreenPixel { bg : RgbColor::new(0, 0, 0), fg : (fg_char, fg_color, fg_style) }
    }
}

#[derive(Component)]
pub struct ManagerReference(pub Entity);

// Player data for managing inputs
#[derive(Component)]
struct PlayerStatus {
    is_sneaking : bool,
    id : u8,
}

// Game data
#[derive(Resource)]
struct GameStatus {
    next_id : u8,
}

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, build)
            .add_systems(Update, (update_screen, init_clients))
            .add_systems(EventLoopUpdate, action)
            // IMPORTANT: you need to register every your game manager like that:
            .register_component_as::<dyn GameManager, StaticGameManager<RainbowGenerator>>()
            .register_component_as::<dyn GameManager, StaticGameManager<MatrixGenerator>>()
            .register_component_as::<dyn GameManager, ScreenBuffer>();
            // in your main function, i dont know if this is a good solution
            // but its best i found yet
    }
}

// Function to spawn screen
pub fn build_screen(
    commands: &mut Commands,
    // layer to spawn screen on
    layer_id: EntityLayerId,
    // position on the left bottom corner
    position: Position,
    // size in screen pixels
    width : u32,
    height : u32,
    // screen pixel size
    pixel_size : u32,
    // whether you want to spawn foreground entities, you may want to use it to reduce amount of entities to 2/3
    enable_fg : bool,
    // thing that draws pixels and manages inputs
    mut manager: impl GameManager + Component
) -> Entity {
    let pixel_bg_scale: f32 = 4.0 / pixel_size as f32;
    let pixel_offset: f64 = 0.125 * pixel_bg_scale as f64;
    let pixel_fg_scale: f32 = pixel_bg_scale / 2.0;

    manager.init(width, height);
    let buffer = ScreenBuffer::reconstruct(&manager, width, height);
    let manager_id = commands.spawn(manager).id();

    // Basically its spawns 3 (or 2 without foreground) text displays per column:
    // 2 for the background and 1 for the foreground.
    // each one of them has line width of 1 so every character is moved to another line,
    // creating vertical line of characters.
    // Background texts are positioned in such way that they
    // cover each other gaps and create continuous line,
    // with that we can modify color at any part of the screen
    // without much performance issues.
    // Third text display is used as foreground, its 2 times smaller than
    // background displays cuz we dont care about gaps, and we can use only 1 display.
    for x in 0..width {
        for i in 0..=1 {
            let mut part = ScreenPart {
                screen_height : height,
                x : x as i32,
                i : 1 - i,
                previous_result : Text::default(),
                ground : Ground::BackGround,
            };
            let result = draw_part(&part, &buffer);
            part.previous_result = result.clone();
            commands.spawn((
                TextDisplayEntityBundle {
                    display_scale : valence::entity::display::Scale([pixel_bg_scale, pixel_bg_scale, pixel_bg_scale].into()),
                    text_display_background : valence::entity::text_display::Background(0),
                    text_display_text_display_flags : valence::entity::text_display::TextDisplayFlags(0b1000),
                    text_display_line_width : valence::entity::text_display::LineWidth(1),
                    text_display_text : valence::entity::text_display::Text(result),
                    layer : layer_id,
                    position : Position::new([
                        // some weird magic numbers to move it since we are using colored characters and not background color
                        position.0[0] as f64 + x as f64 * pixel_offset + 0.05 * pixel_bg_scale as f64,
                        position.0[1] as f64 + i as f64 * pixel_offset - 0.1 * pixel_bg_scale as f64,
                        position.0[2] as f64
                    ] as [f64; 3]),
                    ..Default::default()
                },
                part,
                ManagerReference(manager_id),
            ));
        }

        if enable_fg {
            let mut part = ScreenPart {
                screen_height : height,
                x : x as i32,
                i : 0,
                previous_result : Text::default(),
                ground : Ground::ForeGround,
            };
            let result = draw_part(&part, &buffer);
            part.previous_result = result.clone();
            commands.spawn((
                TextDisplayEntityBundle {
                    display_scale : valence::entity::display::Scale([pixel_fg_scale, pixel_fg_scale, pixel_fg_scale].into()),
                    text_display_background : valence::entity::text_display::Background(0),
                    text_display_text_display_flags : valence::entity::text_display::TextDisplayFlags(0b1000),
                    text_display_line_width : valence::entity::text_display::LineWidth(1),
                    text_display_text : valence::entity::text_display::Text(result),
                    layer : layer_id,
                    position : Position::new([
                        // magic numbers so it doesnt look weird
                        position.0[0] as f64 + (x as f64 + 0.05) * pixel_offset + 0.1 * pixel_fg_scale as f64,
                        // move it down 1 more pixel to hide 1 character
                        position.0[1] as f64 - 1.1 * pixel_offset,
                        // fix z-fighting (if any)
                        position.0[2] as f64 + 0.001
                    ] as [f64; 3]),
                    ..Default::default()
                },
                part,
                ManagerReference(manager_id),
            ));
        }
    }

    return manager_id;
}

fn build(
    mut commands: Commands,
) {
    commands.insert_resource(GameStatus {
        next_id : 0,
    });
}

fn init_clients(
    mut commands: Commands,
    clients: Query<Entity, Added<Client>>,
    mut status: ResMut<GameStatus>,
) {
    for client in clients.iter() {
        commands.get_entity(client).unwrap().insert(PlayerStatus {
            is_sneaking: false,
            id: status.next_id,
        });
        status.next_id += 1;
    }
}

fn update_screen(
    server: Res<Server>,
    mut managers: Query<One<&mut dyn GameManager>>,
    mut screen: Query<(&mut valence::entity::text_display::Text, &mut ScreenPart, &ManagerReference), With<ScreenPart>>,
) {
    // update screen. Doesn't update if it stays the same (to performance?)
    let time = server.current_tick() as f64 / server.tick_rate().get() as f64;
    
    managers.for_each_mut(|mut manager| {
        manager.tick(time);
    });

    for (mut pixel, mut part, manager) in &mut screen {
        let manager = managers.get(manager.0).expect("Screen with no game manager. Forgot to register?");
        let result = draw_part(&part, manager);
        if part.previous_result != result {
            pixel.0 = result.clone();
            part.previous_result = result;
        }
    }
}

// used to draw part of the screen, since its 3 displays per column
fn draw_part(part: &ScreenPart, manager: &dyn GameManager) -> Text {
    let mut result = Text::default();
    if let Ground::BackGround = part.ground {
        for y in 0..(part.screen_height / 2) {
            let pixel = manager.draw(part.x as u32, y * 2 + part.i as u32);
            result = result.add_child(Text::text(BG_PIXEL.to_string()).color(pixel.bg));
        }
    } else {
        for y in 0..(part.screen_height) {
            let pixel = manager.draw(part.x as u32, y);
            if pixel.fg.0 == ' ' {
                // some weird things happen if you use spaces in this thing
                result = result.add_child(Text::text(BIAS_PIXEL.to_string()).color(pixel.bg));
            } else {
                result = result.add_child(pixel.fg.2.resolve(Text::text(pixel.fg.0.to_string()).color(pixel.fg.1)));
            }
        }
        // extra wide character to remove shaking from thin characters
        // (thats why foreground is moved 1 extra pixel down in spawn function)
        result = result.add_child(Text::text(BIAS_PIXEL.to_string()));
    }
    result
}

// Some player inputs. Some of them dont work however because valence updating inventory before i check for item in main hand
fn action(
    mut managers: Query<One<&mut dyn GameManager>>,
    mut clients: Query<(&Client, &ManagerReference, &mut PlayerStatus, &mut Inventory, &mut HeldItem)>,
    mut sneak_event: EventReader<SneakEvent>,
    mut interact_item_event: EventReader<InteractItemEvent>,
    mut hand_swing_event: EventReader<HandSwingEvent>,
    mut packet_event: EventReader<PacketEvent>,
    mut drop_event: EventReader<DropItemStackEvent>,
    mut change_slot_event: EventReader<UpdateSelectedSlotEvent>,
) {
    for event in sneak_event.iter() {
        let (_client, _manager, mut status, _inventory, _held_item) = clients.get_mut(event.client).unwrap();
        if let SneakState::Start = event.state {
            status.is_sneaking = true;
        } else {
            status.is_sneaking = false;
        }
    }

    for event in interact_item_event.iter() {
        // Right click detection
        let (_client, manager, status, inventory, held_item) = clients.get(event.client).unwrap();
        let item = inventory.slot(held_item.slot());
        let Some(nbt) = &item.nbt else {
            continue;
        };
        if !nbt.contains_key("CustomControllerKey") {
            continue;
        }
        let mut manager = managers.get_mut(manager.0).expect("Client with no game manager. Forgot to register?");
        manager.action(status.id, PlayerAction::Secondary(None), status.is_sneaking);
    }

    for event in hand_swing_event.iter() {
        // Left click detection (TODO: fix so it doesnt trigger with drop event at the same time (it doesnt now because item check thing))
        let (_client, manager, status, inventory, held_item) = clients.get(event.client).unwrap();
        let item = inventory.slot(held_item.slot());
        let Some(nbt) = &item.nbt else {
            continue;
        };
        if !nbt.contains_key("CustomControllerKey") {
            continue;
        }
        let mut manager = managers.get_mut(manager.0).expect("Client with no game manager. Forgot to register?");
        manager.action(status.id, PlayerAction::Primary(None), status.is_sneaking);
    }

    for event in packet_event.iter() {
        // I didnt find the event for swapping hands, so heres my solution based on wiki.vg/Protocol
        if event.id != 0x1D || event.data.clone().get_u8() != 6 {
            continue;
        }
        let (_client, manager, status, inventory, held_item) = clients.get(event.client).unwrap();
        let item = inventory.slot(held_item.slot());
        let Some(nbt) = &item.nbt else {
            continue;
        };
        if !nbt.contains_key("CustomControllerKey") {
            continue;
        }
        let mut manager = managers.get_mut(manager.0).expect("Client with no game manager. Forgot to register?");
        manager.action(status.id, PlayerAction::Swap, status.is_sneaking);
    }
    
    for event in drop_event.iter() {
        // Drop event
        let (_client, manager, status, mut inventory, held_item) = clients.get_mut(event.client).unwrap();
        let item = inventory.slot(held_item.slot());
        let Some(nbt) = &item.nbt else {
            continue;
        };
        if !nbt.contains_key("CustomControllerKey") {
            continue;
        }
        let mut manager = managers.get_mut(manager.0).expect("Client with no game manager. Forgot to register?");
        manager.action(status.id, PlayerAction::Drop, status.is_sneaking);

        // Return item back. Doesnt work tho because it cant pass the item check above
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
        if event.slot < 5 {
            continue;
        }
        let (_client, manager, status, inventory, held_item) = clients.get_mut(event.client).unwrap();
        let item = inventory.slot(held_item.slot());
        // doesnt pass this check either
        let Some(nbt) = &item.nbt else {
            continue;
        };
        if !nbt.contains_key("CustomControllerKey") {
            continue;
        }
        let mut manager = managers.get_mut(manager.0).expect("Client with no game manager. Forgot to register?");
        let dir = match event.slot {
            5 => MoveDir::Left,
            6 => MoveDir::Up,
            7 => MoveDir::Down,
            8 => MoveDir::Right,
            _ => unreachable!()
        };
        manager.action(status.id, PlayerAction::SpecialMove(dir), status.is_sneaking);
    }
}
