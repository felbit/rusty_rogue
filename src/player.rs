use crate::prelude::*;

pub struct Player {
    pos: Point,
}

impl Player {
    pub fn new(pos: Point) -> Self {
        Player { pos }
    }

    pub fn render(&self, ctx: &mut BTerm) {
        ctx.set(self.pos.x, self.pos.y, WHITE, BLACK, to_cp437('@'));
    }

    pub fn update(&mut self, ctx: &mut BTerm, map: &Map) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero()
            };

            let new_pos = self.pos + delta;
            if map.can_enter_tile(new_pos) {
                self.pos = new_pos;
            }
        }
    }
}