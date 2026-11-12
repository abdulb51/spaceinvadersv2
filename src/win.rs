use macroquad::prelude::*;
 use crate::utils::preload_image::TextureManager;

pub async fn run(tm: TextureManager, r_score: i32, h_score: i32) -> (String, TextureManager, i32, i32,) {
    
   
   
   
    loop {
        clear_background(DARKGRAY);
        draw_text("Screen 2", 20.0, 40.0, 30.0, WHITE);

        if is_key_pressed(KeyCode::Space) {
            return ("game".to_string(), tm, r_score, h_score);
        }

        next_frame().await;
    }
}