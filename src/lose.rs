use macroquad::prelude::*;
 use crate::utils::preload_image::TextureManager;
use crate::ui::animated_image::AnimatedImage;
use crate::ui::still_image::StillImage;




pub async fn run (tm: TextureManager, h_score: i32) -> (String, TextureManager,  i32,) {
    
    
       
    let bg = StillImage::from_preload(
        tm.get_preload("assets/bg.png").unwrap(),
        920.0,
        920.0,
        0.0,
        0.0,
        true,
        1.0,
    );
    
    
    
    
    
    
    
    
    
    
    
    
    
    loop {
        clear_background(DARKGRAY);
       bg.draw();
         draw_text("your highest score was: ", 0.0, 80.0, 30.0, WHITE);
    draw_text("You died! ", 100.0, 40.0, 30.0, WHITE);
  draw_text("Press R For The Main Menu! ", 0.0, 100.0, 30.0, WHITE);
        if is_key_pressed(KeyCode::R) {
 return ("start".to_string(), tm, h_score);
        }

        next_frame().await;
    }
}
