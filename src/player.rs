use ggez;
use ggez::event;
use ggez::graphics;
use ggez::graphics::Rect;
use ggez::nalgebra as na;
use ggez::timer::sleep;
use std::time::Duration;

pub struct Rec {
    pos_x: f32,
    pos_y: f32,
    width: f32,
    height: f32,
    color: [f32; 4],
    vel: f32,
}

impl Rec {
    pub fn new(
        pos_x: f32,
        pos_y: f32,
        width: f32,
        height: f32,
        color: [f32; 4],
        vel: f32,
    ) -> ggez::GameResult<Rec> {
        let s = Rec {
            pos_x: pos_x,
            pos_y: pos_y,
            width: width,
            height: height,
            color: color,
            vel: vel,
        };
        Ok(s)
    }
    fn selfprint(&self) {
        println!(
            "pos_x:{} pos_y:{} width:{} height:{} color:{:?} vel:{} ",
            self.pos_x, self.pos_y, self.width, self.height, self.color, self.vel
        );
    }
    fn collides_with_ground(&self, other: &Rect) -> bool {
        graphics::Rect {
            x: self.pos_x,
            y: self.pos_y,
            w: self.width,
            h: self.height,
        }
        .overlaps(&other)
    }
    pub fn change_pos(&mut self, gravity: f32) {
        self.pos_y += self.vel;
        self.vel += gravity;
    }

    pub fn apply_force(&mut self, force: f32) {
        self.vel += force;
    }
    pub fn update(&mut self, other: &Rect, force: f32) {
        if self.collides_with_ground(&other) {
            self.vel = 0.0;
            println!("is colliding");
        } else {
            self.change_pos(force);
        }
    }
    pub fn draw(&self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x: self.pos_x,
                y: self.pos_y,
                w: self.width,
                h: self.height,
            },
            self.color.into(),
        )
        .unwrap();
        graphics::draw(ctx, &rect, (na::Point2::new(0.0, 0.0),))?;
        self.selfprint();
        Ok(())
    }
}
