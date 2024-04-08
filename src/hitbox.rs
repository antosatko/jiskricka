use crate::{game::{Coords, Game}, CELL_SIZE};

pub struct Hitbox {
    pub kind: Hitboxes,
    pub hardness: i32,
    pub border_collision: bool,
}

impl Hitbox {
    pub fn new(kind: Hitboxes, hardness: i32) -> Self {
        Self { kind, hardness, border_collision: true }
    }

    pub fn collides(&self, game: &Game) -> bool {
        match self.kind {
            Hitboxes::Rect { x, y, width, height } => {
                let x = x.floor() as i32;
                let y = y.floor() as i32;
                let width = width.floor() as i32;
                let height = height.floor() as i32;
                for i in x..x + width {
                    for j in y..y + height {
                        match game.try_get_cell(i, j) {
                            Some(cell) => {
                                if cell.hardness >= self.hardness {
                                    return true;
                                }
                            }
                            None => {
                                if self.border_collision {
                                    return true;
                                }
                            }
                        }
                    }
                }
                false
            }
            Hitboxes::Circle { x, y, radius } => {
                false
            }
        }
    }

    pub fn move_by(&self, game: &Game, x: f32, y: f32) -> MoveBy {
        match self.kind {
            Hitboxes::Rect { x: hx, y: hy, width, height } => {
                MoveBy { x, y, stop_by: MoveStopBy::Natural }
            }
            Hitboxes::Circle { x: hx, y: hy, radius } => {
                MoveBy { x, y, stop_by: MoveStopBy::Natural }
            }
        }
    }

    pub fn move_point_to(game: &Game, mut point: (f32, f32), mut des: (f32, f32), hardness: i32) -> MoveBy {
        if des.0 < 0.0{
            des.0 = 0.0;
        }
        if des.1 < 0.0{
            des.1 = 0.0;
        }
    
        let start_x = point.0;
        let start_y = point.1;
    

        let vx = des.0 - point.0;
        let vy = des.1 - point.1;
        
        if vx == 0.0 && vy == 0.0 {
            return MoveBy { x: start_x, y: start_y, stop_by: MoveStopBy::Natural }
        }

        let mut grid_x = {
            let a = (point.0 / CELL_SIZE as f32).floor();
            if vx.signum() == -1.0 && point.0 % CELL_SIZE as f32 == 0.0 {
                a - 1.0
            } else {
                a
            }
        };
        let mut grid_y = {
            let a = (point.1 / CELL_SIZE as f32).floor();
            if vy.signum() == -1.0 && point.1 % CELL_SIZE as f32 == 0.0 {
                a - 1.0
            } else {
                a
            }
        };
    
        let sx = vx.signum() != -1.0;
        let sy = vy.signum() != -1.0;
    
        let mut i = 0;
    
        while i < game.width() * game.height() {
            if grid_x >= game.width() as f32 || grid_y >= game.height() as f32 || grid_x < 0.0 || grid_y < 0.0 {
                return MoveBy { x: start_x, y: start_y, stop_by: MoveStopBy::Border }
            }
    
            if game.get_cell(grid_x as i32, grid_y as i32).hardness >= hardness {
                return MoveBy { x: start_x, y: start_y, stop_by: MoveStopBy::Collision }
            }
    
            if Self::is_same_tile(grid_x * CELL_SIZE as f32, grid_y * CELL_SIZE as f32, des.0, des.1) || ((point.0 - des.0).powi(2) + (point.1 - des.1).powi(2)).sqrt() <= 3.0{
                return MoveBy { x: des.0, y: des.1, stop_by: MoveStopBy::Natural }
            }
            /*let px = x % size == 0 ? (sx == 0 ? size : 0) : x % size;
            let py = y % size == 0 ? (sy == 0 ? size : 0) : y % size;*/
            let px = if point.0 % CELL_SIZE as f32 == 0.0 { if sx { CELL_SIZE as f32 } else { 0.0 } } else { point.0 % CELL_SIZE as f32 };
            let py = if point.1 % CELL_SIZE as f32 == 0.0 { if sy { CELL_SIZE as f32 } else { 0.0 } } else { point.1 % CELL_SIZE as f32 };
            // let dx = (sx * CELL_SIZE as f32 - px) / vx;
            // let dy = (sy * CELL_SIZE as f32 - py) / vy;
            let dx = if vx == 0.0 { f32::INFINITY } else { (if sx { CELL_SIZE as f32 } else { 0.0 } - px) / vx };
            let dy = if vy == 0.0 { f32::INFINITY } else { (if sy { CELL_SIZE as f32 } else { 0.0 } - py) / vy };
        
            let d = f32::min(dx, dy);
            point.0 += d * vx;
            point.1 += d * vy;
    
            // fix float erroru
            /*if(dx < dy){
                Math.round(x)
            }
            else{
                Math.round(y)
            }*/
    
            grid_x = {
                let a = (point.0 / CELL_SIZE as f32).floor();
                if vx.signum() == -1.0 && point.0 % CELL_SIZE as f32 == 0.0 {
                    a - 1.0
                } else {
                    a
                }
            };
            grid_y = {
                let a = (point.1 / CELL_SIZE as f32).floor();
                if vy.signum() == -1.0 && point.1 % CELL_SIZE as f32 == 0.0 {
                    a - 1.0
                } else {
                    a
                }
            };
    
            i += 1
        }
    
        return MoveBy { x: start_x, y: start_y, stop_by: MoveStopBy::Natural }
    }

    fn is_same_tile(x: f32, y: f32, x2: f32, y2: f32) -> bool {
        return (x / CELL_SIZE as f32).floor() as u32 == (x2 / CELL_SIZE as f32).floor() as u32 && (y / CELL_SIZE as f32).floor() as u32 == (y2 / CELL_SIZE as f32).floor() as u32
    }
}

pub enum Hitboxes {
    Rect {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
    },
    Circle {
        x: f32,
        y: f32,
        radius: f32,
    },
}

/// When an entity moves, it can collide with cells in the game.
/// 
/// This struct represents the movement of an entity and if it collided with a cell.
/// If yes, the movement will be updated to the point of collision.
#[derive(Debug, Clone, Copy)]
pub struct MoveBy {
    pub x: f32,
    pub y: f32,
    pub stop_by: MoveStopBy,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveStopBy {
    Natural,
    Collision,
    Border,
}