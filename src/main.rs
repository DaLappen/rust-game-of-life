extern crate termion;

use std::{
    io::{self, Read, Write},
    time,
};

use rand::Rng;
use termion::{
    input::MouseTerminal,
    raw::{IntoRawMode, RawTerminal},
};

const HEIGHT: usize = 40;
const WIDTH: usize = 60;

type GameField = [[bool; WIDTH]; HEIGHT];
type Cell = (usize, usize);
struct GameOfLife {
    field: GameField,
}

impl GameOfLife {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();

        let mut field = [[false; WIDTH]; HEIGHT];

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                field[y][x] = rng.gen_ratio(1, 10);
            }
        }
        Self { field }
    }
    fn get_nbors(&self, cell: Cell) -> u8 {
        let offsets = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut nbors = 0;
        for (dy, dx) in offsets {
            let nbor_y = (cell.0 as isize + dy) as usize;
            let nbor_x = (cell.1 as isize + dx) as usize;

            if nbor_y < HEIGHT && nbor_x < WIDTH {
                if self.field[nbor_y][nbor_x] {
                    nbors += 1;
                }
            }
        }
        nbors
    }
    pub fn update(&mut self) {
        let mut shadow_field = self.field;
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                if self.field[y][x] {
                    match self.get_nbors((y, x)) {
                        0..=1 => {
                            shadow_field[y][x] = false;
                        }
                        2..=3 => {}
                        _ => {
                            shadow_field[y][x] = false;
                        }
                    }
                } else {
                    match self.get_nbors((y, x)) {
                        3 => {
                            shadow_field[y][x] = true;
                        }
                        _ => {}
                    }
                }
            }
        }

        self.field = shadow_field;
    }

    pub fn display(&self, stdout: &mut MouseTerminal<RawTerminal<io::Stdout>>) {
        for y in 0..HEIGHT {
            write!(stdout, "{}", termion::cursor::Goto(1, y as u16)).unwrap();
            for x in 0..WIDTH {
                match self.field[y][x] {
                    false => {
                        write!(stdout, "  ",).unwrap();
                    }
                    true => {
                        write!(stdout, "██").unwrap();
                    }
                }
            }
        }
    }
}
fn main() {
    let mut game = GameOfLife::new();

    let mut stdout = termion::input::MouseTerminal::from(io::stdout().into_raw_mode().unwrap());
    stdout.activate_raw_mode().unwrap();
    let mut stdin = termion::async_stdin().bytes();

    write!(
        stdout,
        "{}{}Conways's Game of Life",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    stdout.flush().unwrap();

    std::thread::sleep(time::Duration::from_millis(500));

    'game: loop {
        write!(stdout, "{}", termion::clear::All).unwrap();

        game.update();
        game.display(&mut stdout);

        stdout.flush().unwrap();

        let c = stdin.next();
        if let Some(Ok(b'q')) = c {
            write!(
                stdout,
                "{}{}",
                termion::clear::All,
                termion::cursor::Goto(1, 1)
            )
            .unwrap();
            break 'game;
        }
        std::thread::sleep(time::Duration::from_millis(1000 / 3));
    }
}
