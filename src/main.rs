#![allow(clippy::type_complexity)]
// sowwy
#![allow(dead_code)]

mod screen;
mod game_manager;
mod buffer;

use std::ops::Add;
use buffer::ScreenBuffer;
use screen::ManagerReference;
use valence::{prelude::*, nbt::compound};

const SPAWN_Y: i32 = 64;

// Component used to store player's personal layer
#[derive(Component)]
struct LayerReference(Entity);

pub fn main() {
    App::new()
        .add_plugins((DefaultPlugins, screen::ScreenPlugin))
        .add_systems(
            Update,
            (
                init_clients,
                remove_clients,
            ),
        )
        .run();
}

fn init_clients(
    mut commands: Commands,
    server: Res<Server>,
    dimensions: Res<DimensionTypeRegistry>,
    biomes: Res<BiomeRegistry>,
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
) {
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

        client.send_chat_message("Welcome to Valence! Screens plugin test".italic());

        // create a new layer

        let horizontal_size = 7;
        let vertical_size = 3;
        let screen_pos = BlockPos::new(-3, SPAWN_Y + 1, -10);

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

        let edge_bottom = BlockState::POLISHED_DEEPSLATE_WALL
            .set(PropName::West, PropValue::Tall)
            .set(PropName::East, PropValue::Tall);
        let middle_bottom = BlockState::DEEPSLATE_TILE_WALL
            .set(PropName::West, PropValue::Tall)
            .set(PropName::East, PropValue::Tall)
            .set(PropName::Up, PropValue::False);
        let screen = BlockState::BLACK_STAINED_GLASS_PANE
            .set(PropName::West, PropValue::True)
            .set(PropName::East, PropValue::True);

        for x in 0..horizontal_size {
            layer.chunk.set_block(screen_pos.add([x, 0, 0].into()), middle_bottom);
            for y in 0..vertical_size {
                layer.chunk.set_block(screen_pos.add([x, 1 + y, 0].into()), screen);
            }
        }
        layer.chunk.set_block(screen_pos.add([0, 0, 0].into()), edge_bottom);
        layer.chunk.set_block(screen_pos.add([horizontal_size - 1, 0, 0].into()), edge_bottom);

        let layer_id = commands.spawn(layer).id();
        commands.get_entity(entity).unwrap().insert(LayerReference(layer_id));

        // ↑ you dont need to do it, single layer in build() function from examples is fine,
        // i just wanted to try doing something like this

        client_layer_id.0 = layer_id;
        visible_chunk_layer.0 = layer_id;
        visible_entity_layers.0.insert(layer_id);

        // pixels per block half, works fine only with powers of 2 for some reason
        let pixel_size = 4;
        // Screen size in pixels = size in blocks * (2 * pixel size)
        let width = horizontal_size as u32 * 2 * pixel_size;
        let height = vertical_size as u32 * 2 * pixel_size;
        let manager = screen::build_screen(
            &mut commands,
            EntityLayerId(layer_id),
            Position([
                screen_pos.x as f64,
                screen_pos.y as f64 + 1.0,
                screen_pos.z as f64 + 9.0 / 16.0
            ].into()),
            width,
            height,
            pixel_size,
            true,
            // Game manager
            ScreenBuffer::load_image("valence.png", width, height, true)
        );

        // required for inputs
        commands.get_entity(entity).unwrap().insert(ManagerReference(manager));
        inventory.as_mut().set_slot(36, ItemStack::new(
            ItemKind::IronHorseArmor,
            1,
            Some(compound! {
                "display" => compound! {
                    "Name" => "{\"text\":\"Controller\",\"italic\":false}",
                },
                // Custom tag for controller
                "CustomControllerKey" => compound! {}
            })
        ));
    }
}

fn remove_clients(
    mut clients: RemovedComponents<Client>,
    entities: Query<(Entity, &EntityLayerId)>,
    world: &World,
    mut commands: Commands,
) {
    fn despawn(commands: &mut Commands, entity: Entity) {
        if let Some(mut command) = commands.get_entity(entity) {
            command.despawn();
        }
    }

    // despawn player's personal layer and all entities on it
    for client in clients.iter() {
        if let Some(layer) = world.get::<LayerReference>(client) {
            despawn(&mut commands, layer.0);
            for (entity, layer_id) in entities.iter() {
                if layer_id.0 == layer.0 {
                    despawn(&mut commands, entity);
                }
            }
        }
    }
}
