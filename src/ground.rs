use std::time::Duration;
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;


struct Bottom {
    color: [f32; 4],
}


impl Bottom {
    fn new(color: [f32; 4]) -> ggez::GameResult<Bottom> {
        let s = Bottom { color: color };
        Ok(s)
    }
    fn draw(&self, ctx: &mut ggez::Context) {}
}
