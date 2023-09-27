#![allow(dead_code)]

mod screen;
mod manager;

use valence::message::ChatMessageEvent;
use valence::prelude::*;
use std::ops::Add;
use bevy_trait_query::RegisterExt;

use crate::screen::buffer::ScreenBuffer;
use crate::screen::game_manager::GameManager;
use crate::screen::input::GameData;
use crate::manager::rainbow_game_manager::RainbowGameManager;
use crate::manager::matrix_game_manager::MatrixGameManager;
use crate::manager::paint_game_manager::PaintGameManager;
use crate::manager::snake_game_manager::SnakeGameManager;
use crate::screen::Screen;

const SPAWN_Y: i32 = 64;

pub fn main() {
    App::new()
        .add_plugins((DefaultPlugins, screen::ScreenPlugin))
        .add_systems(Startup, build)
        .add_systems(Update, (despawn_disconnected_clients, init_clients, chat))
        // You need to register all your game managers like this:
        .register_component_as::<dyn GameManager, ScreenBuffer>()
        .register_component_as::<dyn GameManager, MatrixGameManager>()
        .register_component_as::<dyn GameManager, RainbowGameManager>()
        .register_component_as::<dyn GameManager, SnakeGameManager>()
        .register_component_as::<dyn GameManager, PaintGameManager>()
        .run();
}

fn build(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
) {
    const HORIZONTAL_SIZE: u32 = 7;
    const VERTICAL_SIZE: u32 = 3;
    const SCREEN_POS: BlockPos = BlockPos::new(-3, SPAWN_Y + 1, -10);

    let mut layer = LayerBundle::new(ident!("overworld"), &dimensions, &biomes, &server);

    for z in -5..5 {
        for x in -5..5 {
            layer.chunk.insert_chunk([x, z], UnloadedChunk::new());
        }
    }

    for z in -16..16 {
        for x in -16..16 {
            layer.chunk.set_block([x, SPAWN_Y, z], BlockState::STONE);
        }
    }
    layer.chunk.set_block([0, SPAWN_Y, 0], BlockState::COBBLESTONE);

    let edge_bottom_block = BlockState::POLISHED_DEEPSLATE_WALL
        .set(PropName::West, PropValue::Tall)
        .set(PropName::East, PropValue::Tall);
    let middle_bottom_block = BlockState::DEEPSLATE_TILE_WALL
        .set(PropName::West, PropValue::Tall)
        .set(PropName::East, PropValue::Tall)
        .set(PropName::Up, PropValue::False);
    let screen_block = BlockState::BLACK_STAINED_GLASS_PANE
        .set(PropName::West, PropValue::True)
        .set(PropName::East, PropValue::True);

    for x in 0..HORIZONTAL_SIZE as i32 {
        layer.chunk.set_block(SCREEN_POS.add([x, 0, 0].into()), middle_bottom_block);
        for y in 0..VERTICAL_SIZE as i32 {
            layer.chunk.set_block(SCREEN_POS.add([x, 1 + y, 0].into()), screen_block);
        }
    }
    layer.chunk.set_block(SCREEN_POS.add([0, 0, 0].into()), edge_bottom_block);
    layer.chunk.set_block(SCREEN_POS.add([HORIZONTAL_SIZE as i32 - 1, 0, 0].into()), edge_bottom_block);

    let layer_id = commands.spawn(layer).id();

    // pixels per block half, works fine only with powers of 2 for some reason
    let pixel_size = 4;
    let width = HORIZONTAL_SIZE * 2 * pixel_size;
    let height = VERTICAL_SIZE * 2 * pixel_size;
    let _ = screen::build_screen(
        &mut commands,
        EntityLayerId(layer_id),
        Position([
            SCREEN_POS.x as f64,
            SCREEN_POS.y as f64 + 1.0,
            SCREEN_POS.z as f64 + 9.0 / 16.0
        ].into()),
        width,
        height,
        pixel_size,
        true,
        // Game manager
        // ScreenBuffer::load_image("valence.png", width, height, true)
        PaintGameManager::default()
    );
}

fn init_clients(
    mut commands: Commands,
    mut data: ResMut<GameData>,
    mut clients: Query<
        (
            Entity,
            &mut Client,
            &mut EntityLayerId,
            &mut VisibleChunkLayer,
            &mut VisibleEntityLayers,
            &mut Position,
            &mut GameMode,
            &mut Inventory,
        ),
        Added<Client>,
    >,
    layers: Query<Entity, (With<EntityLayer>, With<ChunkLayer>)>,
    screens: Query<(Entity, &Screen)>,
) {
    let layer = layers.single();
    let (screen_id, _screen) = screens.single();

    for (
        entity,
        mut client,
        mut client_layer_id,
        mut visible_chunk_layer,
        mut visible_entity_layers,
        mut pos,
        mut game_mode,
        mut inventory,
    ) in &mut clients {
        pos.set([0.0, SPAWN_Y as f64 + 1.0, 0.0]);
        *game_mode = GameMode::Adventure;
        client_layer_id.0 = layer;
        visible_chunk_layer.0 = layer;
        visible_entity_layers.0.insert(layer);

        client.send_chat_message("Welcome to Valence! Screens plugin".italic());

        // required for inputs
        screen::input::init_client(&mut commands, &mut data, entity, screen_id);
        inventory.as_mut().set_slot(36, screen::input::get_controller_item());
    }
}

fn chat(
    mut chat_message_event: EventReader<ChatMessageEvent>,
    usernames: Query<&Username>,
    mut clients: Query<&mut Client>,
) {
    for event in chat_message_event.iter() {
        let username = &usernames.get(event.client).unwrap().0;
        let message = &*event.message;
        let message_text = Text::text(" ")
            .add_child(username)
            .add_child(Text::text(": ").color(Color::GRAY))
            .add_child(message.to_string());

        for mut client in clients.iter_mut() {
            client.as_mut().send_chat_message(message_text.clone());
        }
    }
}
