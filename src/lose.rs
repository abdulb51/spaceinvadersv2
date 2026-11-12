use macroquad::prelude::*;
 use crate::utils::preload_image::TextureManager;
use crate::ui::animated_image::AnimatedImage;
use crate::ui::still_image::StillImage;

pub async fn run(tm: TextureManager, s_score: i32, h_score: i32) -> (String, TextureManager, i32, i32,) {
    loop {
        clear_background(WHITE);
        draw_text("You Lose!", 20.0, 40.0, 30.0, WHITE);

        if is_key_pressed(KeyCode::Space) {
             return ("start".to_string(), tm, s_score, h_score);
        }

        next_frame().await;
    }
}