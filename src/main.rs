use macroquad::prelude::*;

const SPRITE_WIDTH: f32 = 96.0;
const SPRITE_HEIGHT: f32 = 96.0;
const NUM_FRAMES: usize = 7;
const SCALE: f32 = 2.0;

#[macroquad::main("Running Person Animation")]
async fn main() {
    // Load assets
    let background = load_texture("/Users/lazycodebaker/Desktop/spritesheet/3.png")
        .await
        .unwrap();
    let spritesheet = load_texture("/Users/lazycodebaker/Desktop/spritesheet/Run.png")
        .await
        .unwrap();

    background.set_filter(FilterMode::Linear);
    spritesheet.set_filter(FilterMode::Nearest);

    loop {
        clear_background(WHITE);

        // Get actual screen size dynamically
        let window_width = screen_width();
        let window_height = screen_height();

        // Draw background filling the entire window
        draw_texture_ex(
            &background,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(window_width, window_height)), // Stretch to fill the screen
                ..Default::default()
            },
        );

        // Animation update
        static mut CURRENT_FRAME: usize = 0;
        static mut ANIMATION_TIMER: f32 = 0.0;

        unsafe {
            ANIMATION_TIMER += get_frame_time() * 10.0;
            if ANIMATION_TIMER >= 1.0 {
                ANIMATION_TIMER = 0.0;
                CURRENT_FRAME = (CURRENT_FRAME + 1) % NUM_FRAMES;
            }

            // Center the sprite
            let character_x = (window_width - (SPRITE_WIDTH * SCALE)) / 2.0;
            let character_y = ((window_height - (SPRITE_HEIGHT * SCALE)) / 2.0)  + 100.0;

            draw_texture_ex(
                &spritesheet,
                character_x,
                character_y,
                WHITE,
                DrawTextureParams {
                    source: Some(Rect {
                        x: (CURRENT_FRAME as f32) * SPRITE_WIDTH,
                        y: 0.0,
                        w: SPRITE_WIDTH,
                        h: SPRITE_HEIGHT,
                    }),
                    dest_size: Some(vec2(SPRITE_WIDTH * SCALE, SPRITE_HEIGHT * SCALE)),
                    ..Default::default()
                },
            );
        }

        // Exit on ESC key
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
