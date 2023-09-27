use crate::screen::buffer::ScreenBuffer;
use crate::screen::input::PlayerAction;

// Thing responsible for drawing screen and responding to inputs
#[bevy_trait_query::queryable]
pub trait GameManager: Send + Sync + 'static {
    // init game manager with size of the screen. Called only once in screen spawn function
    fn init(&mut self, width: u32, height: u32, has_fg: bool);

    // draw screen pixel at given position. x, y is always 0..width, 0..height
    fn draw(&self) -> ScreenBuffer;

    // called every tick
    fn tick(&mut self);

    // player action with player uid, action and whether player is sneaking (for extra functionality)
    fn action(&mut self, player: u8, action: PlayerAction);
}
