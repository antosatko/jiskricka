use sfml::graphics::{RenderTarget, Shape, Transformable};
use rand::prelude::*;

use crate::mask;

pub struct Game {
    cells: Vec<Cell>,
    width: u32,
    height: u32,
    /// How many cells will be randomly updated each iteration.
    pub iteration: u32,
}


impl Game {
    pub fn new(width: u32, height: u32) -> Game {
        Game {
            cells: vec![Cell::default(); (width * height) as usize],
            width,
            height,
            iteration: crate::ITERATON,
        }
    }

    pub fn draw(&self, rw: &mut sfml::graphics::RenderWindow) {
        let cell_size = crate::CELL_SIZE;
        let mut shape = sfml::graphics::RectangleShape::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let cell = self.get_cell(x as i32, y as i32);
                shape.set_size((cell_size as f32, cell_size as f32));
                shape.set_position((x as f32 * cell_size as f32, y as f32 * cell_size as f32));
                shape.set_fill_color(cell.color());
                rw.draw(&shape);
            }
        }
    }

    pub fn update(&mut self, frame: &mut Frame) {
        let mut rng = rand::thread_rng();
        for _ in 0..self.iteration {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            let cell = self.get_cell(x as i32, y as i32);
            let (x, y) = (x as i32, y as i32);
            Cell::update(self, Coords { x, y }, cell, frame);
            self.apply_frame(frame);
        }
        
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns the cell at the given coordinates, if it exists.
    pub fn try_get_cell(&self, x: i32, y: i32) -> Option<&Cell> {
        if self.cell_exists(x, y){
            let (x, y) = (x as u32, y as u32);
            Some(&self.cells[(y * self.width + x) as usize])
        } else {
            None
        }
    }

    /// Returns the cell at the given coordinates or panics if it doesn't exist.
    pub fn get_cell(&self, x: i32, y: i32) -> &Cell {
        let (x, y) = (x as u32, y as u32);
        &self.cells[(y * self.width + x) as usize]
    }

    /// Sets the cell at the given coordinates.
    pub fn try_set_cell(&mut self, x: i32, y: i32, cell: Cell) {
        if self.cell_exists(x, y) {
            let (x, y) = (x as u32, y as u32);
            self.cells[(y * self.width + x) as usize] = cell;
        }
    }

    /// Sets the cell at the given coordinates or panics if it doesn't exist.
    pub fn set_cell(&mut self, x: i32, y: i32, cell: Cell) {
        let (x, y) = (x as u32, y as u32);
        self.cells[(y * self.width + x) as usize] = cell;
    }
    
    pub fn cell_exists(&self, x: i32, y: i32) -> bool {
        x < self.width as i32 && y < self.height as i32 && x >= 0 && y >= 0
    }

    /// Returns an iterator over the cells in the game that are masked by the given mask.
    pub fn iter_masked_cells<'a>(&'a self, mask: &'a Mask) -> MaskedCellIterator<'a> {
        MaskedCellIterator {
            mask,
            game_width: self.width,
            game_height: self.height,
            current_index: 0,
            game: self,
        }
    }

    /// Exhaustively applies the actions in the frame to the game.
    pub fn apply_frame(&mut self, frame: &mut Frame) {
        while let Some(action) = frame.poll() {
            match action {
                Action::Cell(coords, cell_action) => {
                    match cell_action {
                        CellAction::Set(cell) => self.try_set_cell(coords.x, coords.y, cell),
                        CellAction::Clear => self.set_cell(coords.x, coords.y, Cell::default()),
                        CellAction::SetKind(kind) => self.set_cell(coords.x, coords.y, Cell::new(kind)),
                        CellAction::SetColorMode(color_mode) => {
                            let mut cell = self.get_cell(coords.x, coords.y).clone();
                            cell.color_mode = color_mode;
                            self.set_cell(coords.x, coords.y, cell);
                        }
                    }
                }
                Action::Swap(coords1, coords2) => {
                    let cell1 = self.get_cell(coords1.x, coords1.y).clone();
                    let cell2 = self.get_cell(coords2.x, coords2.y).clone();
                    self.set_cell(coords1.x, coords1.y, cell2);
                    self.set_cell(coords2.x, coords2.y, cell1);
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub kind: Cells,
    pub color_mode: ColorMode,
}

impl Cell {
    pub fn new(kind: Cells) -> Cell {
        Cell { kind, color_mode: kind.color_mode() }
    }
}

impl Default for Cell {
    fn default() -> Cell {
        let kind = Cells::Air;
        Cell {
            kind,
            color_mode: kind.color_mode(),
        }
    }
}

impl Cell {
    pub fn color(&self) -> sfml::graphics::Color {
        match self.color_mode {
            ColorMode::Static(color) => color,
            ColorMode::Dynamic => self.kind.color(),
        }
    }

    pub fn update(game: &Game, coords: Coords, cell: &Cell, frame: &mut Frame) {
        match cell.kind {
            Cells::Air => (),
            Cells::Wall => (),
            Cells::Sand => {
                let below_coords = Coords { x: coords.x, y: coords.y + 1 };
                if let Some(below_cell) = game.try_get_cell(below_coords.x, below_coords.y) {
                    if below_cell.kind.hardness() < cell.kind.hardness() {
                        frame.add_action(Action::Swap(coords, below_coords));
                    } else {
                        let mut rng = rand::thread_rng();
                        let direction = rng.gen_range(0..2);
                        match direction {
                            0 => {
                                let left_coords = Coords { x: coords.x - 1, y: coords.y + 1 };
                                let cell = match game.try_get_cell(left_coords.x, left_coords.y) {
                                    Some(cell) => cell,
                                    None => return,
                                };
                                if cell.kind.hardness() < cell.kind.hardness() {
                                    frame.add_action(Action::Swap(coords, left_coords));
                                }
                            }
                            1 => {
                                let right_coords = Coords { x: coords.x + 1, y: coords.y + 1 };
                                let cell = match game.try_get_cell(right_coords.x, right_coords.y) {
                                    Some(cell) => cell,
                                    None => return,
                                };
                                if cell.kind.hardness() < cell.kind.hardness() {
                                    frame.add_action(Action::Swap(coords, right_coords));
                                }
                            }
                            _ => (),
                        }
                    }
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Cells {
    Air,
    Wall,
    Sand,
}

impl Cells {
    pub fn color(&self) -> sfml::graphics::Color {
        match self {
            Cells::Air => sfml::graphics::Color::WHITE,
            Cells::Wall => sfml::graphics::Color::BLACK,
            Cells::Sand => sfml::graphics::Color::YELLOW,
        }
    }

    pub fn color_mode(&self) -> ColorMode {
        match self {
            Cells::Air => ColorMode::Static(self.color()),
            Cells::Wall => ColorMode::Static(self.color()),
            Cells::Sand => ColorMode::Static(self.color()),
        }
    }

    pub fn hardness(&self) -> u32 {
        match self {
            Cells::Air => 2,
            Cells::Wall => 100,
            Cells::Sand => 100,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ColorMode {
    Static(sfml::graphics::Color),
    Dynamic,
}

#[derive(Debug, Copy, Clone)]
pub struct Coords {
    pub x: i32,
    pub y: i32,
}



#[derive(Clone, Copy)]
pub struct Mask {
    data: &'static [bool],
    stride: u32,
    pub center: Coords,
    pub coords: Coords,
}

impl Mask {
    pub const fn new(data: &'static [bool], stride: u32, center: Coords) -> Option<Mask> {
        if data.len() as u32 % stride != 0 {
            return None;
        }
        Some(Mask { data, stride, center, coords: Coords { x: 0, y: 0 } })
    }

    pub fn get(&self, x: u32, y: u32) -> bool {
        self.data[(y * self.stride + x) as usize]
    }
}

impl Default for Mask {
    fn default() -> Mask {
        Mask { data: &[], stride: 0, center: Coords { x: 0, y: 0 }, coords: Coords { x: 0, y: 0 } }
    }
}

impl std::fmt::Debug for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Mask {{ stride: {}, data: {:?} }}", self.stride, self.data)
    }
}

impl std::fmt::Display for Mask {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let width = self.stride;
        let height = self.data.len() as u32 / self.stride;
        for y in 0..height {
            for x in 0..width {
                write!(f, "{}", if self.data[(y * width + x) as usize] { '*' } else { ' ' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct MaskedCellIterator<'a> {
    pub mask: &'a Mask,
    pub game_width: u32,
    pub game_height: u32,
    pub current_index: usize,
    pub game: &'a Game,
}

impl<'a> Iterator for MaskedCellIterator<'a> {
    type Item = (Coords, &'a Cell);

    fn next(&mut self) -> Option<Self::Item> {
        while self.current_index < self.mask.data.len() {
            let row = self.mask.coords.y - self.mask.center.y + (self.current_index / self.mask.stride as usize) as i32;
            let col = self.mask.coords.x - self.mask.center.x + (self.current_index % self.mask.stride as usize) as i32;
            self.current_index += 1;
            if self.mask.data[self.current_index - 1] 
                && row < self.game_height as i32 
                && row >= 0
                && col < self.game_width as i32
                && col >= 0
            {
                let coords = Coords { x: col, y: row };
                let cell = self.game.get_cell(col, row);
                return Some((coords, cell));
            }
        }
        None
    }
}

pub struct Frame {
    actions: Vec<Action>,
}

impl Frame {
    pub fn new() -> Frame {
        Frame {
            actions: Vec::new(),
        }
    }

    pub fn add_action(&mut self, action: Action) {
        self.actions.push(action);
    }

    pub fn clear(&mut self) {
        self.actions.clear();
    }

    pub fn is_empty(&self) -> bool {
        self.actions.is_empty()
    }

    pub fn len(&self) -> usize {
        self.actions.len()
    }

    pub fn poll(&mut self) -> Option<Action> {
        self.actions.pop()
    }
}

pub enum Action {
    Cell(Coords, CellAction),
    Swap(Coords, Coords),
}

pub enum CellAction {
    Set(Cell),
    SetKind(Cells),
    SetColorMode(ColorMode),
    Clear,
}