use std::collections::VecDeque;

use macroquad::{
    input::is_key_pressed,
    math::{Vec2, vec2},
    prelude::KeyCode,
    rand::RandomRange,
    window::{screen_height, screen_width},
};

pub const TILE: f32 = 30.;
pub struct GameBoard {
    pub delay: f64,
    pub playing: bool,
    dir: Vec2,
    snake: VecDeque<Vec2>,
    food: Vec2,
}

impl GameBoard {
    pub fn new() -> GameBoard {
        let mut deq = VecDeque::new();
        deq.push_front(vec2(5., 5.));
        GameBoard {
            delay: 0.17,
            playing: true,
            dir: vec2(1., 0.),
            snake: deq,
            food: vec2(7., 7.),
        }
    }

    pub fn init(&mut self) {
        *self = GameBoard::new();
    }
    pub fn snake(&self) -> &VecDeque<Vec2> {
        &self.snake
    }
    pub fn food(&self) -> &Vec2 {
        &self.food
    }
    pub fn is_playing(&self) -> bool {
        self.playing
    }
    pub fn handle_click(&mut self) {
        if is_key_pressed(KeyCode::Left) && self.dir != vec2(1., 0.) {
            self.dir = vec2(-1., 0.);
        }
        if is_key_pressed(KeyCode::Right) && self.dir != vec2(-1., 0.) {
            self.dir = vec2(1., 0.);
        }
        if is_key_pressed(KeyCode::Up) && self.dir != vec2(0., 1.) {
            self.dir = vec2(0., -1.);
        }
        if is_key_pressed(KeyCode::Down) && self.dir != vec2(0., -1.) {
            self.dir = vec2(0., 1.);
        }
    }

    pub fn update_snake(&mut self, score: &mut u32) {
        let max_x = (screen_width() / TILE) as i32;
        let max_y = (screen_height() / TILE) as i32;
        let mut new_head: Vec2 = self.snake[0] + self.dir;
        if new_head.x < 0. {
            new_head.x = max_x as f32 - 1.;
        }
        if new_head.x >= max_x as f32 {
            new_head.x = 0.;
        }

        if new_head.y < 0. {
            new_head.y = max_y as f32 - 1.;
        }
        if new_head.y >= max_y as f32 {
            new_head.y = 0.;
        }
        if self.snake.contains(&new_head) {
            self.playing = false;
            return;
        }
        self.snake.push_front(new_head);
        if new_head == self.food {
            self.food = Vec2 {
                x: RandomRange::gen_range(0, max_x) as f32,
                y: RandomRange::gen_range(0, max_y) as f32,
            };
            *score += 1;
            self.delay -= 0.005;
        } else {
            self.snake.pop_back();
        }
    }
}
