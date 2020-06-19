use ggez;
use ggez::graphics;
use ggez::nalgebra as na;

pub struct Bottom {
    pos_x: f32,
    pos_y: f32,
    width: f32,
    height: f32,
    color: [f32; 4],
}

impl Bottom {
    pub fn new(
        pos_x: f32,
        pos_y: f32,
        width: f32,
        height: f32,
        color: [f32; 4],
    ) -> ggez::GameResult<Bottom> {
        let s = Bottom {
            pos_x: pos_x,
            pos_y: pos_y,
            width: width,
            height: height,
            color: color,
        };
        Ok(s)
    }
    pub fn return_rect(&self) -> graphics::Rect {
        graphics::Rect {
            x: self.pos_x,
            y: self.pos_y,
            w: self.width,
            h: self.height,
        }
    }
    fn selfprint(&self) {
        println!(
            "ground = pos_x:{} pos_y:{} width:{} height:{} color:{:?}",
            self.pos_x, self.pos_y, self.width, self.height, self.color
        );
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
        graphics::draw(ctx, &rect, (na::Point2::new(self.pos_x, self.pos_y),))?;
        self.selfprint();
        Ok(())
    }
}
