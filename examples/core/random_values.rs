use raylib::*;

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    let rl = raylib::init()
        .title("raylib [core] example - generate random values")
        .size(screen_width, screen_height)
        .build();

    let mut frames_counter = 0;
    let mut rand_value = rl.get_random_value(-8, 5);
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        frames_counter += 1;

        if frames_counter / 120 % 2 == 1 {
            rand_value = rl.get_random_value(-8, 5);
            frames_counter = 0;
        }

        rl.begin_drawing();
        rl.clear_background(Color::RAYWHITE);
        rl.draw_text("Every 2 seconds a new random value is generated:", 130, 100, 20, Color::MAROON);
        rl.draw_text(format!("{}", rand_value).as_str(), 360, 180, 80, Color::LIGHTGRAY);
        rl.end_drawing();
    }
}
