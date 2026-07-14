#![windows_subsystem = "windows"]
use std::ops::Range;

use platform_compat::ppath;
use raylib::consts::{KeyboardKey::*, MouseButton::*};
use raylib::prelude::*;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

struct Ball {
    position: Vector2,
    speed: f32,
    radius: f32,
    color: Color,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Hello, World")
        .vsync()
        .build();

    rl.set_target_fps(60);

    println!("raylib initialized!");

    let mut ball = Ball {
        position: Vector2 {
            x: SCREEN_WIDTH / 2.0,
            y: SCREEN_HEIGHT / 2.0,
        },
        speed: 100.0,
        radius: 10.0,
        color: Color::BLUE,
    };

    let mut value: i32 = rl.get_random_value(-100..100);
    let mut frame_count: f32 = 0.0;

    let cow =
        Image::load_image(&ppath!("assets/images/cow.png")).expect("Failed to load image of cow");
    let _ = rl
        .load_texture(&thread, &ppath!("assets/images/cow.png"))
        .expect("Failed to load cow texture");
    let cow_tex = rl
        .load_texture_from_image(&thread, &cow)
        .expect("Failed to load texture from image");

    let audio = raylib::core::audio::RaylibAudio::init_audio_device().unwrap();
    let music = audio
        .new_music(&ppath!("assets/sounds/Night in Venice.mp3"))
        .unwrap();
    music.play_stream();

    println!("All set up!");

    while !rl.window_should_close() {
        // game_loop::run(rl, thread, 60, move |rl, thread| {
        /*  --- UPDATE --- */
        let dt = rl.get_frame_time();
        frame_count += dt;

        if frame_count > 1.0 {
            value = rl.get_random_value(Range {
                start: -100,
                end: 100,
            });
            frame_count -= 1.0;
        }

        #[cfg(not(target_os = "emscripten"))]
        if rl.is_key_down(KEY_ESCAPE) {
            rl.request_quit();
        }

        if rl.is_key_down(KEY_RIGHT) {
            ball.position.x += ball.speed * dt
        }
        if rl.is_key_down(KEY_LEFT) {
            ball.position.x -= ball.speed * dt
        }
        if rl.is_key_down(KEY_DOWN) {
            ball.position.y += ball.speed * dt
        }
        if rl.is_key_down(KEY_UP) {
            ball.position.y -= ball.speed * dt
        }
        if rl.is_mouse_button_down(MOUSE_BUTTON_LEFT) {
            ball.position = ball
                .position
                .lerp(rl.get_mouse_position(), ball.speed * dt / 100.0);
        }

        if rl.is_key_pressed(KEY_SPACE) {
            if ball.color == Color::BLUE {
                ball.color = Color::RED;
            } else if ball.color == Color::RED {
                ball.color = Color::GREEN;
            } else if ball.color == Color::GREEN {
                ball.color = Color::YELLOW;
            } else {
                ball.color = Color::BLUE;
            }
        }

        music.update_stream();

        /* --- DRAW --- */
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::WHITE);
        d.draw_texture(&cow_tex, 0, 0, Color::WHITE);
        draw_text_center(&mut d, "Hello, world!", 15, 20, Color::BLACK);
        draw_text_center(&mut d, format!("{}", value).as_str(), 30, 20, Color::BLUE);
        d.draw_circle_v(ball.position, ball.radius, ball.color);
        // });
    }
}

fn draw_text_center(d: &mut RaylibDrawHandle, text: &str, y: i32, font_size: i32, color: Color) {
    let text_length = d.measure_text(text, font_size);

    d.draw_text(
        text,
        (SCREEN_WIDTH as i32) / 2 - (text_length / 2),
        y,
        font_size,
        color,
    );
}
