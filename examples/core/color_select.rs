use raylib::*;
use raylib::consts::*;

fn main() {
    let screen_width = 800;
    let screen_height = 450;

    let rl = raylib::init()
        .title("raylib [core] example - color selection (collision detection)")
        .size(screen_width, screen_height)
        .build();

    let mut colors = [
        Color::DARKGRAY,
        Color::MAROON,
        Color::ORANGE,
        Color::DARKGREEN,
        Color::DARKBLUE,
        Color::DARKPURPLE,
        Color::DARKBROWN,
        Color::GRAY,
        Color::RED,
        Color::GOLD,
        Color::LIME,
        Color::BLUE,
        Color::VIOLET,
        Color::BROWN,
        Color::LIGHTGRAY,
        Color::PINK,
        Color::YELLOW,
        Color::GREEN,
        Color::SKYBLUE,
        Color::PURPLE,
        Color::BEIGE,
    ];

    let mut colors_recs = [Rectangle { x: 0.0, y: 0.0, width: 0.0, height: 0.0 }; 21];

    for (i, rect) in colors_recs.iter_mut().enumerate() {
        rect.x = (20 + 100 * (i % 7) + 10 * (i % 7)) as f32;
        rect.y = (60 + 100 * (i / 7) + 10 * (i / 7)) as f32;
        rect.width = 100.0;
        rect.height = 100.0;
    }

    let mut selected = [false; 21];
    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut mouse_point = rl.get_mouse_position();

        rl.clear_background(Color::RAYWHITE);

        for (i, rect) in colors_recs.iter_mut().enumerate() {
            if rl.check_collision_point_rec(mouse_point, *rect) {
                colors[i].a = 120;
                if rl.is_mouse_button_pressed(MOUSE_LEFT_BUTTON) {
                    selected[i] = !selected[i];
                    println!("Selected: {}", selected[i]);
                }
            } else {
                colors[i].a = 255;
            }
        }

        rl.begin_drawing();
        for (i, rect) in colors_recs.iter_mut().enumerate() {
            rl.draw_rectangle_rec(*rect, colors[i]);

            if selected[i] {
                rl.draw_rectangle(rect.x as i32, rect.y as i32, 100, 10, Color::RAYWHITE);
                rl.draw_rectangle(rect.x as i32, rect.y as i32, 10, 100, Color::RAYWHITE);
                rl.draw_rectangle(rect.x as i32 + 90, rect.y as i32, 10, 100, Color::RAYWHITE);
                rl.draw_rectangle(rect.x as i32, rect.y as i32 + 90, 100, 10, Color::RAYWHITE);
            }
        }
        rl.end_drawing();
    }
}
