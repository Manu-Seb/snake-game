use macroquad::prelude::*;
use snake_game;
use snake_game::GameBoard;

const TILE: f32 = 30.;
fn draw_objects(board: &GameBoard, score: &u32) {
    let scoretext = score.to_string();
    draw_text(&scoretext, 20.0, 20.0, 40.0, WHITE);
    for s in board.snake() {
        draw_rectangle(s.x * TILE, s.y * 30., 30., 30., GREEN);
    }
    draw_rectangle(
        board.get_food().x * TILE,
        board.get_food().y * TILE,
        TILE,
        TILE,
        RED,
    );
}

fn death(conf: &mut GameBoard, score: &mut u32) {
    let scoretext = score.to_string();
    clear_background(RED);
    draw_text("Your score is :", 20.0, 20.0, 40.0, WHITE);
    draw_text("PRESS SPACE TO TRY AGAIN", 20.0, 100.0, 60.0, WHITE);
    if is_key_pressed(KeyCode::Space) {
        conf.init();
        *score = 0;
    }
    draw_text(&scoretext, 20.0, 60.0, 40.0, WHITE);
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut time = get_time();
    let mut score: u32 = 0;

    let mut board = GameBoard::new();
    loop {
        clear_background(BLACK);
        if board.playing() {
            board.handle_click();
            draw_objects(&board, &score);
            if get_time() - time > board.delay() {
                time = get_time();
                board.update_snake(&mut score);
            }
        } else {
            time = get_time();
            death(&mut board, &mut score);
        }
        next_frame().await
    }
}
