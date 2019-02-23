use raylib::*;
use raylib::consts::*;

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    let rl = raylib::init()
        .title("raylib [core] example - mouse input")
        .size(screen_width, screen_height)
        .build();

    let mut ball_color = Color::DARKBLUE;
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let ball_position = rl.get_mouse_position();

        if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) {
            ball_color = Color::MAROON
        } else if rl.is_mouse_button_pressed(MOUSE_MIDDLE_BUTTON) {
            ball_color = Color::LIME
        } else if rl.is_mouse_button_pressed(MOUSE_RIGHT_BUTTON) {
            ball_color = Color::DARKBLUE
        }

        rl.begin_drawing();
        rl.clear_background(Color::RAYWHITE);
        rl.draw_text("move ball with mouse and click mouse button to change color", 10, 10, 20, Color::DARKGRAY);
        rl.draw_circle_v(ball_position, 40.0, ball_color);
        rl.end_drawing();
    }
}
