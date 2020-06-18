use std::time::Duration;
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::timer::sleep;
mod rectangle;
use rectangle::Rec;

struct MainState {
    background: [f32; 4],
    player: Rec,
    ground: Bottom,
    gravity: f32,
}

// struct Rec {
//     pos_x: f32,
//     pos_y: f32,
//     width: f32,
//     height: f32,
//     color: [f32; 4],
//     vel: f32,
// }

struct Bottom {
    color: [f32; 4],
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
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult {
        self.player.change_pos(self.gravity);

        sleep(Duration::new(0, 5));
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        graphics::clear(ctx, self.background.into());
        self.player.draw(ctx)?;
        graphics::present(ctx)?;
        Ok(())
    }
}

// impl Rec {
//     fn new(
//         pos_x: f32,
//         pos_y: f32,
//         width: f32,
//         height: f32,
//         color: [f32; 4],
//         vel: f32,
//     ) -> ggez::GameResult<Rec> {
//         let s = Rec {
//             pos_x: pos_x,
//             pos_y: pos_y,
//             width: width,
//             height: height,
//             color: color,
//             vel: vel,
//         };
//         Ok(s)
//     }

//     fn change_pos(&mut self, gravity: f32) {
//         self.pos_y += self.vel;
//         self.vel += gravity;
//     }

//     fn apply_force(&mut self, force: f32) {
//         self.vel += force;
//     }
//     fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
//         let rect = graphics::Mesh::new_rectangle(
//             ctx,
//             graphics::DrawMode::fill(),
//             graphics::Rect {
//                 x: self.pos_x,
//                 y: self.pos_y,
//                 w: self.width,
//                 h: self.height,
//             },
//             self.color.into(),
//         )
//         .unwrap();
//         graphics::draw(ctx, &rect, (na::Point2::new(self.pos_x,self.pos_y),))?;
//         Ok(())
//     }
// }
impl Bottom {
    fn new(color: [f32; 4]) -> ggez::GameResult<Bottom> {
        let s = Bottom { color: color };
        Ok(s)
    }
    fn draw(&self, ctx: &mut ggez::Context) {}
}

pub fn main() -> ggez::GameResult {
    let cb = ggez::ContextBuilder::new("platformer", "jacob");
    let (ctx, event_loop) = &mut cb.build()?;
    let player = Rec::new(30.0, 30.0, 50.0, 50.0, [1.0, 0.6, 0.2, 1.0], -10.0);
    let ground = Bottom::new([0.0, 0.0, 1.0, 1.0]);
    let state = &mut MainState::new([0.0, 1.0, 0.0, 1.0], player.unwrap(), ground.unwrap(), 1.0)?;
    event::run(ctx, event_loop, state)
}
