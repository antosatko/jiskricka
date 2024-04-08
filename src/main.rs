pub mod mask;
pub mod game;
pub mod hitbox;


use game::Game;
use sfml::graphics::RenderTarget;

use crate::game::Coords;

const WIN_WIDTH: u32 = 1600;
const WIN_HEIGHT: u32 = 900;

const CELLS_WIDTH: u32 = 100;
const CELLS_HEIGHT: u32 = 100;

const CELL_SIZE: u32 = WIN_WIDTH / CELLS_WIDTH;

const ITERATON: u32 = CELLS_HEIGHT * CELLS_WIDTH / 10;

fn main() {
    let mut game = Game::new(CELLS_WIDTH, CELLS_HEIGHT);
    let mut frame = game::Frame::new();

    let mut rw = sfml::graphics::RenderWindow::new(
        sfml::window::VideoMode::new(WIN_WIDTH, WIN_HEIGHT, 32),
        "SFML Example",
        sfml::window::Style::DEFAULT,
        &sfml::window::ContextSettings::default(),
    );
    rw.set_vertical_sync_enabled(true);
    rw.set_framerate_limit(60);

    let mut star = mask::FAR_MASK.clone();
    star.coords.x += 3;
    star.coords.y += 1;
    for (coords, cell) in game.iter_masked_cells(&star) {
        frame.add_action(game::Action::Cell(coords, game::CellAction::Set(game::Cell::new(game::Cells::Sand))));
    }
    for i in 0..20 {
        frame.add_action(game::Action::Cell(game::Coords { x: i as i32, y: 20 }, game::CellAction::Set(game::Cell::new(game::Cells::Wall))));
    }
    game.apply_frame(&mut frame);

    use std::time::Instant;
    let mut last_frame = Instant::now();

    let a = (0.5, 0.5);
    let b = (100.0, 250.0);

    let c = hitbox::Hitbox::move_point_to(&game, a, b, 50);
    println!("{:?}", c);

    loop {
        let current_frame = Instant::now();
        let delta = current_frame.duration_since(last_frame).as_secs_f32();
        last_frame = current_frame;
        let fps = 1.0 / delta;

        while let Some(event) = rw.poll_event() {
            match event {
                sfml::window::Event::Closed => return,
                _ => {}
            }
        }
        rw.clear(sfml::graphics::Color::BLACK);
        game.update(&mut frame);
        game.draw(&mut rw);
        rw.display();
    }
}
