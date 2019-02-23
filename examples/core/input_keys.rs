use raylib::*;
use raylib::consts::*;

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    let rl = raylib::init()
        .title("raylib [core] example - keyboard input")
        .size(screen_width, screen_height)
        .build();

    let mut ball_position = Vector2::new((screen_width as f32) / 2.0, (screen_height as f32) / 2.0);
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        if rl.is_key_down(KEY_RIGHT) { ball_position.x += 2.0 }
        if rl.is_key_down(KEY_LEFT) { ball_position.x -= 2.0 }
        if rl.is_key_down(KEY_UP) { ball_position.y -= 2.0 }
        if rl.is_key_down(KEY_DOWN) { ball_position.y += 2.0 }

        rl.begin_drawing();
        rl.clear_background(Color::RAYWHITE);
        rl.draw_text("move the ball with arrow keys", 10, 10, 20, Color::DARKGRAY);
        rl.draw_circle_v(ball_position, 50.0, Color::MAROON);
        rl.end_drawing();
    }
}
