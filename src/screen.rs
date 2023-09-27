pub mod game_manager;
pub mod buffer;
pub mod input;
pub mod pixel;

use valence::prelude::*;
use bevy_trait_query::One;
use valence::entity::text_display::TextDisplayEntityBundle;

use buffer::ScreenBuffer;
use game_manager::GameManager;
use crate::screen::input::Uid;
use crate::screen::pixel::BG_PIXEL;
use crate::screen::pixel::BIAS_PIXEL;

#[derive(Clone)]
enum Ground {
    BackGround,
    ForeGround,
}

#[derive(Component)]
pub struct Screen {
    pub parts : Vec<Entity>,
    pub position : Position,
    pub width : u32,
    pub height : u32,
    pub pixel_size : u32,
    pub manager : Entity,
    pub occupied_uids : [bool; Uid::MAX as usize],
    pub next_free_uid : Uid,
}

impl Screen {
    pub fn next_uid(&mut self) -> Uid {
        let uid = self.next_free_uid;
        self.occupied_uids[self.next_free_uid as usize] = true;
        while self.occupied_uids[self.next_free_uid as usize] {
            self.next_free_uid += 1;
        }
        uid
    }

    pub fn free_uid(&mut self, uid: Uid) {
        self.occupied_uids[uid as usize] = false;
        if uid < self.next_free_uid {
            self.next_free_uid = uid;
        }
    }
}

#[derive(Component)]
struct ScreenPart {
    pub x : i32,
    pub i : i32,
    pub previous_state: Text,
    pub ground : Ground,
}

impl ScreenPart {
    pub fn draw(&mut self, buffer: &ScreenBuffer, height: u32) -> Text {
        let mut result = Text::default();
        if let Ground::BackGround = self.ground {
            for y in 0..(height / 2) {
                let pixel = buffer.get(self.x as u32, y * 2 + self.i as u32).unwrap_or_default();
                result = result.add_child(Text::text(BG_PIXEL.to_string()).color(pixel.bg));
            }
        } else {
            for y in 0..height {
                let pixel = buffer.get(self.x as u32, y).unwrap_or_default();
                if pixel.fg.0 == ' ' {
                    // some weird things happen if you use spaces in this thing
                    result = result.add_child(Text::text(BIAS_PIXEL.to_string()).color(pixel.bg));
                } else {
                    result = result.add_child(pixel.fg.2.apply(Text::text(pixel.fg.0.to_string()).color(pixel.fg.1)));
                }
            }
            // extra wide character to remove shaking from thin characters
            // (thats why foreground is moved 1 extra pixel down in spawn function)
            result = result.add_child(Text::text(BIAS_PIXEL.to_string()));
        }
        result
    }
}

pub struct ScreenPlugin;

impl Plugin for ScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_screen);
        input::build(app);
    }
}

// Function to spawn screen
pub fn build_screen(
    commands: &mut Commands,
    // layer to spawn screen on
    layer_id: EntityLayerId,
    // position on the left bottom(not sure) corner
    position: Position,
    // size in screen pixels
    width : u32,
    height : u32,
    // screen pixel size
    pixel_size : u32,
    // whether you want to spawn foreground entities, you may want to use it to reduce amount of entities to 2/3
    enable_fg : bool,
    mut manager: impl GameManager + Component
) -> Entity {
    let pixel_bg_scale: f32 = 4.0 / pixel_size as f32;
    let pixel_offset: f64 = 0.125 * pixel_bg_scale as f64;
    let pixel_fg_scale: f32 = 0.5 * pixel_bg_scale;

    manager.init(width, height, enable_fg);
    let buffer = manager.draw();
    let manager = commands.spawn(manager).id();

    let mut parts: Vec<Entity> = vec![];

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
                x : x as i32,
                i : 1 - i,
                previous_state: Text::default(),
                ground : Ground::BackGround,
            };
            let result = part.draw(&buffer, height);
            part.previous_state = result.clone();
            let part = commands.spawn((
                TextDisplayEntityBundle {
                    display_scale : valence::entity::display::Scale([pixel_bg_scale, pixel_bg_scale, pixel_bg_scale].into()),
                    text_display_background : valence::entity::text_display::Background(0),
                    text_display_text_display_flags : valence::entity::text_display::TextDisplayFlags(0b1000),
                    text_display_line_width : valence::entity::text_display::LineWidth(1),
                    text_display_text : valence::entity::text_display::Text(result),
                    layer : layer_id,
                    position : Position::new([
                        // some weird magic numbers to move it since we are using colored characters and not background color
                        position.0[0] + x as f64 * pixel_offset + 0.05 * pixel_bg_scale as f64,
                        position.0[1] + i as f64 * pixel_offset - 0.1 * pixel_bg_scale as f64,
                        position.0[2]
                    ]),
                    ..Default::default()
                },
                part,
            )).id();
            parts.push(part);
        }

        if enable_fg {
            let mut part = ScreenPart {
                x : x as i32,
                i : 0,
                previous_state: Text::default(),
                ground : Ground::ForeGround,
            };
            let result = part.draw(&buffer, height);
            part.previous_state = result.clone();
            let part = commands.spawn((
                TextDisplayEntityBundle {
                    display_scale : valence::entity::display::Scale([pixel_fg_scale, pixel_fg_scale, pixel_fg_scale].into()),
                    text_display_background : valence::entity::text_display::Background(0),
                    text_display_text_display_flags : valence::entity::text_display::TextDisplayFlags(0b1000),
                    text_display_line_width : valence::entity::text_display::LineWidth(1),
                    text_display_text : valence::entity::text_display::Text(result),
                    layer : layer_id,
                    position : Position::new([
                        // magic numbers so it doesnt look weird
                        position.0[0] + (x as f64 + 0.05) * pixel_offset + 0.1 * pixel_fg_scale as f64,
                        // move it down 1 more pixel to hide 1 character
                        position.0[1] - 1.1 * pixel_offset,
                        // fix z-fighting (if any)
                        position.0[2] + 0.001
                    ]),
                    ..Default::default()
                },
                part,
            )).id();
            parts.push(part);
        }
    }

    commands.spawn(Screen {
        parts,
        position,
        width,
        height,
        pixel_size,
        manager,
        occupied_uids: [false; Uid::MAX as usize],
        next_free_uid: 0,
    });

    return manager;
}

fn update_screen(
    mut managers: Query<One<&mut dyn GameManager>>,
    mut screens: Query<&mut Screen>,
    mut screen_parts: Query<(&mut valence::entity::text_display::Text, &mut ScreenPart)>,
) {
    managers.for_each_mut(|mut manager| {
        manager.tick();
    });

    for screen in &mut screens {
        let manager = managers.get(screen.manager).expect("Screen with no game manager. Forgot to register?");
        let buffer = manager.draw();
        for part_id in screen.parts.iter() {
            let (mut text, mut part) = screen_parts.get_mut(*part_id).unwrap();
            let result = part.draw(&buffer, screen.height);
            if part.previous_state != result {
                text.0 = result.clone();
                part.previous_state = result;
            }
        }
    }
}
