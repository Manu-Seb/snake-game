use macroquad::{
    input::is_key_pressed,
    math::{Vec2, vec2},
    prelude::KeyCode,
    rand::RandomRange,
    window::{screen_height, screen_width},
};

pub struct Config {}

pub struct GameBoard {
    delay: f64,
    playing: bool,
    dir: Vec2,
    snake: Vec<Vec2>,
    food: Vec2,
}

impl GameBoard {
    pub fn new() -> GameBoard {
        GameBoard {
            delay: 0.17,
            playing: true,
            dir: vec2(1., 0.),
            snake: vec![vec2(5., 5.)],
            food: vec2(7., 7.),
        }
    }

    pub fn init(&mut self) {
        self.delay = 0.17;
        self.playing = true;
        self.dir = vec2(1., 0.);
        self.snake = vec![vec2(5., 5.)];
        self.food = vec2(7., 7.);
    }
    pub fn get_delay(&self) -> f64 {
        self.delay
    }
    pub fn get_playing(&self) -> bool {
        self.playing
    }
    pub fn get_snake(&self) -> &Vec<Vec2> {
        &self.snake
    }
    pub fn get_food(&self) -> &Vec2 {
        &self.food
    }
    pub fn get_dir(&self) -> &Vec2 {
        &self.dir
    }
    pub fn stop_playing(&mut self) {
        self.playing = false;
    }
    pub fn update_food(&mut self) {
        self.food = Vec2 {
            x: RandomRange::gen_range(0, 15) as f32,
            y: RandomRange::gen_range(0, 15) as f32,
        }
    }
    pub fn add_snake_body_end(&mut self, new_head: Vec2) {
        self.snake.insert(0, new_head);
    }
    pub fn reduce_snake_body(&mut self) {
        self.snake.pop();
    }
    pub fn check_edge(&mut self) {
        let max_x = (screen_width() / 30.) as i32;
        let max_y = (screen_height() / 30.) as i32;
        if self.snake[0].x < 0. {
            self.snake[0].x = max_x as f32 - 1.;
        }
        if self.snake[0].x >= max_x as f32 {
            self.snake[0].x = 0.;
        }

        if self.snake[0].y < 0. {
            self.snake[0].y = max_y as f32 - 1.;
        }
        if self.snake[0].y >= max_y as f32 {
            self.snake[0].y = 0.;
        }
    }
}

pub fn handle_click(board: &mut GameBoard) {
    if is_key_pressed(KeyCode::Left) {
        board.dir = vec2(-1., 0.);
    }
    if is_key_pressed(KeyCode::Right) {
        board.dir = vec2(1., 0.);
    }
    if is_key_pressed(KeyCode::Up) {
        board.dir = vec2(0., -1.);
    }
    if is_key_pressed(KeyCode::Down) {
        board.dir = vec2(0., 1.);
    }
}
