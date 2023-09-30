use crate::screen::buffer::ScreenBuffer;
use crate::screen::input::{PlayerAction, Uid};

// Thing responsible for drawing screen and responding to inputs
#[bevy_trait_query::queryable]
pub trait GameManager: Send + Sync + 'static {
    fn init(&mut self, width: u32, height: u32, has_fg: bool);

    fn draw(&self) -> ScreenBuffer;

    fn tick(&mut self);

    fn action(&mut self, player: Uid, action: PlayerAction);
}
