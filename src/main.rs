use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::timer::sleep;
use std::time::Duration;

mod player;
use player::Rec;

mod ground;
use ground::Bottom;

struct MainState {
    background: [f32; 4],
    player: Rec,
    ground: Bottom,
    gravity: f32,
}

impl MainState {
    fn new(
        background: [f32; 4],
        player: Rec,
        ground: Bottom,
        gravity: f32,
    ) -> ggez::GameResult<MainState> {
        let s = MainState {
            background: background,
            player: player,
            ground: ground,
            gravity: gravity,
        };
        Ok(s)
    }
}
impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        self.player.update(&self.ground.return_rect(), self.gravity);
        sleep(Duration::new(0, 5));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, self.background.into());
        self.player.draw(ctx)?;
        self.ground.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("platformer", "jacob");
    let (ctx, event_loop) = &mut cb.build()?;
    let player = Rec::new(70.0, 30.0, 50.0, 50.0, [1.0, 0.6, 0.2, 1.0], 1.0);
    let ground = Bottom::new(0.0, 200.0, 800.0, 200.0, [0.0, 0.0, 1.0, 1.0]);
    let state = &mut MainState::new([0.0, 1.0, 0.0, 1.0], player.unwrap(), ground.unwrap(), 0.05)?;
    event::run(ctx, event_loop, state)
}
