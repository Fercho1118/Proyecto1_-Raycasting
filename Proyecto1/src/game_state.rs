use raylib::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum GameState {
    Welcome,
    Playing,
    Victory,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Difficult,
}

impl Difficulty {
    pub fn get_maze_file(&self) -> &str {
        match self {
            Difficulty::Easy => "maze_easy.txt",
            Difficulty::Medium => "maze_medium.txt", 
            Difficulty::Difficult => "maze_difficult.txt",
        }
    }
    
    pub fn get_name(&self) -> &str {
        match self {
            Difficulty::Easy => "EASY",
            Difficulty::Medium => "MEDIUM",
            Difficulty::Difficult => "DIFFICULT",
        }
    }
}

pub struct GameManager {
    pub state: GameState,
    pub current_difficulty: Difficulty,
    pub selected_option: usize, //Para navegación en el menú
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            state: GameState::Welcome,
            current_difficulty: Difficulty::Easy,
            selected_option: 0,
        }
    }
    
    pub fn reset_to_welcome(&mut self) {
        self.state = GameState::Welcome;
        self.selected_option = 0;
    }
    
    pub fn start_game(&mut self, difficulty: Difficulty) {
        self.current_difficulty = difficulty;
        self.state = GameState::Playing;
    }
    
    pub fn win_game(&mut self) {
        self.state = GameState::Victory;
    }
}
