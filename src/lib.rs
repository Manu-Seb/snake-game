use macroquad::{
    input::is_key_pressed,
    math::{Vec2, vec2},
    prelude::KeyCode,
    rand::RandomRange,
    window::{screen_height, screen_width},
};

const TILE: f32 = 30.;

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
    pub fn playing(&self) -> bool {
        self.playing
    }
    pub fn get_food(&self) -> &Vec2 {
        &self.food
    }
    pub fn delay(&self) -> f64 {
        self.delay
    }
    pub fn snake(&self) -> &Vec<Vec2> {
        &self.snake
    }
    pub fn check_edge(&mut self) {
        let max_x = (screen_width() / TILE) as i32;
        let max_y = (screen_height() / TILE) as i32;
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
    pub fn handle_click(&mut self) {
        if is_key_pressed(KeyCode::Left) && self.dir != vec2(1., 0.) {
            self.dir = vec2(-1., 0.);
        }
        if is_key_pressed(KeyCode::Right) && self.dir != vec2(-1., 0.){
            self.dir = vec2(1., 0.);
        }
        if is_key_pressed(KeyCode::Up) && self.dir != vec2(0., 1.){
            self.dir = vec2(0., -1.);
        }
        if is_key_pressed(KeyCode::Down) && self.dir != vec2(0., -1.){
            self.dir = vec2(0., 1.);
        }
    }

    pub fn update_snake(&mut self, score: &mut u32) {
        let new_head: Vec2 = self.snake[0] + self.dir;
        if new_head == self.food {
            self.snake.insert(0, new_head);
            self.check_edge();
            self.food = Vec2 {
                x: RandomRange::gen_range(0, 15) as f32,
                y: RandomRange::gen_range(0, 15) as f32,
            };
            *score += 1;
        } else {
            if self.snake.contains(&new_head) {
                self.playing = false;
            }
            self.snake.insert(0, new_head);

            self.check_edge();
            self.snake.pop();
        }
    }
}
