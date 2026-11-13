use macroquad::prelude::*;
 use crate::utils::preload_image::TextureManager;

pub async fn run(tm: TextureManager, h_score: i32) -> (String, TextureManager, i32,) {
    
   
   
   
    loop {
        clear_background(DARKGRAY);
        draw_text("Screen 2", 20.0, 40.0, 30.0, WHITE);

        if is_key_pressed(KeyCode::Space) {
            return ("game".to_string(), tm, h_score);
        }

        next_frame().await;
    }
}