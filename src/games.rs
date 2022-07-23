pub enum GameState {
    New,
    Play,
    GameOver,
    Reset,
}

const INITIAL_SPEED: f32 = 200.0;
const INITIAL_ASTEROIDS: u8 = 10;

pub struct Game {
    pub state: GameState,
    pub speed: f32,
    pub prev_score: u16,
    pub next_score: u16,
    pub asteroids: u8,
    pub powerups: u8,
    pub ticks: u32,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::New,
            speed: INITIAL_SPEED,
            prev_score: 0,
            next_score: 0,
            asteroids: INITIAL_ASTEROIDS,
            powerups: 1,
            ticks: 0,
        }
    }
}
