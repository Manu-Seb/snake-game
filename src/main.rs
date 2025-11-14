use macroquad::{prelude::*, rand::RandomRange};

#[macroquad::main("MyGame")]
async fn main() {
    let delay: f64 = 0.17;
    let mut time = get_time();
    let mut food = vec2(7., 7.);

    let mut dir = vec2(1., 0.);
    let mut snake = vec![vec2(5., 5.)];

    let mut score = 0;
    let mut playing = true;


let max_x = (screen_width() / 30.) as i32;
let max_y = (screen_height() / 30.) as i32;
    loop {
        clear_background(BLACK);
        let scoretext = score.to_string();
        draw_text(&scoretext, 20.0, 20.0, 40.0, WHITE);
        if playing {
            if is_key_pressed(KeyCode::Left) {
                dir = vec2(-1., 0.);
            }
            if is_key_pressed(KeyCode::Right) {
                dir = vec2(1., 0.);
            }
            if is_key_pressed(KeyCode::Up) {
                dir = vec2(0., -1.);
            }
            if is_key_pressed(KeyCode::Down) {
                dir = vec2(0., 1.);
            }

            for s in &snake {
                draw_rectangle(s.x * 30., s.y * 30., 30., 30., GREEN);
            }
            draw_rectangle(food.x * 30., food.y * 30., 30., 30., RED);
            if get_time() - time > delay {
                time = get_time();
                if snake[0] + dir == food {
                    snake.insert(0, snake[0] + dir);

                    score += 1;
                    food = Vec2 {
                        x: RandomRange::gen_range(0, 15) as f32,
                        y: RandomRange::gen_range(0, 15) as f32,
                    }
                } else {
                    let mut i = snake.len() - 1;
                    while i > 0 {
                        snake[i] = snake[i - 1];
                        i -= 1;
                    }
                    snake[0] = snake[0] + dir;

                    if snake[0].x < 0. {
                        snake[0].x = max_x as f32 - 1.;
                    }
                    if snake[0].x >= max_x as f32 {
                        snake[0].x = 0.;
                    }

                    if snake[0].y < 0. {
                        snake[0].y = max_y as f32 - 1.;
                    }
                    if snake[0].y >= max_y as f32 {
                        snake[0].y = 0.;
                    }

                    let mut count = 0;
                    for s in &snake {
                        if *s == snake[0] {
                            count += 1;
                        }
                        if count > 1 {
                            playing = false;
                        }
                    }
                }
            }
        } else {
            time = get_time();
            clear_background(RED);
            draw_text("Your score is :", 20.0, 20.0, 40.0, WHITE);
            draw_text(&scoretext, 20.0, 60.0, 40.0, WHITE);
            draw_text("PRESS SPACE TO TRY AGAIN", 20.0, 100.0, 60.0, WHITE);
            if is_key_pressed(KeyCode::Space) {
                playing = true;
                food = vec2(7., 7.);
                score = 0;
                dir = vec2(1., 0.);
                snake = vec![vec2(5., 5.)];
            }
        }
        next_frame().await
    }
}
