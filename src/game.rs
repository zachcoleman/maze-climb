use rand::{thread_rng, Rng};

use crate::maze::Maze;

pub enum GameStatus {
    Alive,
    Dead,
}

#[derive(Debug)]
pub struct Game {
    pub level: usize,
    pub lives: usize,
}

impl Game {
    pub fn new() -> Self {
        Self { level: 1, lives: 3 }
    }

    /// programmatically generate mazes based on level
    pub fn get_maze(&self) -> Maze {
        // based on current level -> generate random MxN maze
        // w/ simple formula:
        // level <= 10 -> 0.6 * level + 4
        // level >  10 -> 0.2 * level + 4
        let mut tgt: usize = ((0.6 * self.level as f32) as usize) + 4;
        if self.level > 10 {
            tgt = ((0.6 * self.level as f32) as usize) + 4;
        }
        let diff: usize = usize::max((0.4 * tgt as f32) as usize, 1);
        let (m, n) = (
            thread_rng().gen_range((tgt - diff)..(tgt + diff)),
            thread_rng().gen_range((tgt - diff)..(tgt + diff)),
        );
        let max_depth = thread_rng().gen_range(6..(m * n));

        let mut maze = Maze::new(m, n);
        maze.cut_up_maze(max_depth);
        maze
    }

    pub fn apply_loss(&mut self) -> GameStatus {
        if self.lives > 1 {
            self.lives -= 1;
            GameStatus::Alive
        } else {
            GameStatus::Dead
        }
    }

    pub fn apply_win(&mut self) {
        self.level += 1;
    }
}
