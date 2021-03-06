pub trait GameOfLife {

    /// Return `Some(true)` if the cell is alive, `Some(false)` if it is dead, or `None` if `x`
    /// and/or `y` are out of bounds.
    fn is_cell_alive(&self, x: i32, y: i32) -> Option<bool>;

    /// Swap the given cell from alive to dead or dead to alive.
    ///
    /// If `x` or `y` is out of bounds, this method should do nothing.
    ///
    /// The origin is assumed to be at the top left, i.e. when `(x, y) == (0, 0)` then the top-left-most
    /// cell should be toggled.
    fn toggle_cell(&mut self, x: i32, y: i32);

    /// Execute one timestep; i.e. cause cells to live, be born, or die based on the amount of
    /// neighbors they have.
    fn tick(&mut self);

    /// Return the current width in cells of the game.
    fn width(&self) -> i32;

    /// Return the current height in cells of the game.
    fn height(&self) -> i32;
}

/// A blatantly-wrong implementation of GameOfLife, to show the syntax for implementing traits.
///
/// You can start off your own implementation by copy-pasting this.
pub struct BrokenGame {
    cell_state: bool,
}

impl BrokenGame {
    pub fn new(game_width: i32, game_height: i32) -> BrokenGame { // note `new` is just a regular function
        assert!(game_width > 0, "game width must be greater than 0");
        assert!(game_height > 0, "game height must be greater than 0");
        BrokenGame { cell_state: true }
    }
}

impl GameOfLife for BrokenGame {
    fn is_cell_alive(&self, _x: i32, _y: i32) -> Option<bool> {
        Some(self.cell_state)
    }

    fn toggle_cell(&mut self, _x: i32, _y: i32) { // underscores stop compiler complaining about unused variables
        // Toggle the only cell we have
        self.cell_state = !self.cell_state;
    }

    fn tick(&mut self) {
        self.cell_state = !self.cell_state;

        println!(
            "Broken game tick completed - cell_state is now {}",
            self.cell_state
        );
    }

    fn width(&self) -> i32 {
        49 // broken implementation always returns the same width
    }

    fn height(&self) -> i32 {
        40 // broken implementation always returns the same height
    }
}

pub struct Mine {
    height: i32,
    width: i32,
    cell_states: Vec<Vec<bool>>,
}

impl Mine {
    pub fn new(game_width: i32, game_height: i32) -> Self {
        Mine {
            width: game_width,
            height: game_height,
            cell_states: vec![vec![false; game_width as usize]; game_height as usize]
        }
    }

    fn get_live_neighbor_count(&mut self, x: i32, y: i32) -> i32 {
        let mut count = 0;
        let xs = vec![x-1, x, x+1];
        let ys = vec![y-1, y, y+1];

        for yi in ys.iter() {
            for xi in xs.iter() {
                if *xi == x && *yi == y {
                    continue;
                }
                match self.is_cell_alive(*xi, *yi) {
                    Some(is_alive) => {
                        if is_alive {
                            count = count + 1;
                        }
                    },
                    None => {},
                }
            }
        }
        count
    }
}

impl GameOfLife for Mine {
    /// Return `Some(true)` if the cell is alive, `Some(false)` if it is dead, or `None` if `x`
    /// and/or `y` are out of bounds.
    fn is_cell_alive(&self, x: i32, y: i32) -> Option<bool> {
        if x < 0 || y < 0 || x > self.width as i32 || y > self.height as i32 {
            None
        } else {
            Some(self.cell_states[y as usize][x as usize])
        }
    }

    /// Swap the given cell from alive to dead or dead to alive.
    ///
    /// If `x` or `y` is out of bounds, this method should do nothing.
    ///
    /// The origin is assumed to be at the top left, i.e. when `(x, y) == (0, 0)` then the top-left-most
    /// cell should be toggled.
    fn toggle_cell(&mut self, _x: i32, _y: i32) {
        let cell = self.is_cell_alive(_x, _y);
        match cell {
            Some(is_alive) => {
                if is_alive {
                    self.cell_states[_y as usize][_x as usize] = false
                } else {
                    self.cell_states[_y as usize][_x as usize] = true
                }
            },
            None => { },
        }
    }

    /// Execute one timestep; i.e. cause cells to live, be born, or die based on the amount of
    /// neighbors they have.
    fn tick(&mut self) {

        let mut next_board = vec![vec![false; self.width as usize]; self.height as usize];

        for y in 0..self.height as i32-1 {
            for x in 0..self.width as i32-1 {
                let neighbors = self.get_live_neighbor_count(x,y);
                match self.is_cell_alive(x, y) {
                    Some(is_alive) => {
                        if is_alive {
                            if neighbors < 2 {
                                next_board[y as usize][x as usize] = false;
                            } else if neighbors == 2 || neighbors == 3 {
                                next_board[y as usize][x as usize] = true;
                            } else {
                                next_board[y as usize][x as usize] = false;
                            }
                        } else if neighbors == 3 {
                            next_board[y as usize][x as usize] = true;
                        } else {
                            next_board[y as usize][x as usize] = false;
                        }
                    },
                    None => {},
                }
            }
        }
        self.cell_states = next_board;
    }

    /// Return the current width in cells of the game.
    fn width(&self) -> i32 {
        self.width
    }

    /// Return the current height in cells of the game.
    fn height(&self) -> i32 {
        self.height
    }
}

mod broken_game_test {
    use super::{BrokenGame, GameOfLife};

    /// A basic test to show you how to write tests in Rust, in case you want to write your own.
    #[test]
    fn broken_game_is_definitely_broken() {
        let mut game = BrokenGame::new();
        let cell_0_0_orig_val = game.is_cell_alive(0, 0);

        // change a totally different cell from 0,0
        game.toggle_cell(1, 1);

        let cell_0_0_new_val = game.is_cell_alive(0, 0);
        // now we expect cell 0,0's liveness to have changed because we know that BrokenGame
        // is a totally broken implementation. If the two values are equal, then something seriously
        // weird is going on.
        // Tip: `assert_ne!` means "assert not equal" - normally using `assert!` or `assert_eq!` is typical.
        assert_ne!(cell_0_0_orig_val, cell_0_0_new_val, "Uh oh, cell 0,0 failed to change from its \
        original value even though we tried to mutate another cell, so BrokenGame is may not be \
        broken anymore!?");
    }
}
