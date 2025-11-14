use macroquad::prelude::*;
use snake_game;
use snake_game::GameBoard;

fn draw_objects(board: &GameBoard) {
    for s in board.get_snake() {
        draw_rectangle(s.x * 30., s.y * 30., 30., 30., GREEN);
    }
    draw_rectangle(
        board.get_food().x * 30.,
        board.get_food().y * 30.,
        30.,
        30.,
        RED,
    );
}

fn update_snake(board: &mut GameBoard, score: &mut u32) {
    let food = board.get_food();
    let dir = board.get_dir();
    if board.get_snake()[0] + *dir == *food {
        let new_head: Vec2 = board.get_snake()[0] + *dir;

        board.add_snake_body_end(new_head);
        *score += 1;
        board.update_food();
    } else {
        let new_head: Vec2 = board.get_snake()[0] + *dir;

        if board.get_snake().contains(&new_head) {
            board.stop_playing();
        }
        board.add_snake_body_end(new_head);
        board.reduce_snake_body();

        board.check_edge();
    }
}

fn death(conf: &mut GameBoard, score: &mut u32) {
    clear_background(RED);
    draw_text("Your score is :", 20.0, 20.0, 40.0, WHITE);
    draw_text("PRESS SPACE TO TRY AGAIN", 20.0, 100.0, 60.0, WHITE);
    if is_key_pressed(KeyCode::Space) {
        conf.init();
        *score = 0;
    }
}

#[macroquad::main("MyGame")]
async fn main() {
    let mut time = get_time();
    let mut score: u32 = 0;

    let mut board = GameBoard::new();
    loop {
        clear_background(BLACK);
        let scoretext = score.to_string();
        draw_text(&scoretext, 20.0, 20.0, 40.0, WHITE);
        if board.get_playing() {
            snake_game::handle_click(&mut board);
            draw_objects(&board);
            if get_time() - time > board.get_delay() {
                time = get_time();
                update_snake(&mut board, &mut score);
            }
        } else {
            time = get_time();
            death(&mut board, &mut score);
            draw_text(&scoretext, 20.0, 60.0, 40.0, WHITE);
        }
        next_frame().await
    }
}
