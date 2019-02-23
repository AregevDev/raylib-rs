use raylib::*;

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    let rl = raylib::init()
        .title("raylib [core] example - mouse wheel")
        .size(screen_width, screen_height)
        .build();

    let mut box_position_y = screen_height / 2 - 40;
    let scroll_speed = 4;
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        box_position_y -= rl.get_mouse_wheel_move() * scroll_speed;

        rl.begin_drawing();
        rl.clear_background(Color::RAYWHITE);
        rl.draw_rectangle(screen_width / 2 - 40, box_position_y, 80, 80, Color::MAROON);
        rl.draw_text("Use mouse wheel to move the cube up and down!", 10, 10, 20, Color::DARKGRAY);
        rl.draw_text(format!("Box position Y {}", box_position_y).as_str(), 10, 40, 20, Color::DARKGRAY);
        rl.end_drawing();
    }
}
