use raylib::*;

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    let rl = raylib::init()
        .title("raylib [core] example - basic window")
        .size(screen_width, screen_height)
        .build();

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        rl.begin_drawing();
        rl.clear_background(Color::RAYWHITE);
        rl.draw_text("Congrats! You created your first window!", 190, 200, 20, Color::LIGHTGRAY);
        rl.end_drawing();
    }
}
