
use crate::ui::text_button::TextButton;

    use crate::ui::still_image::StillImage;
use macroquad::prelude::*;
use crate::utils::preload_image::TextureManager;
pub async fn run (tm: TextureManager, h_score: i32) -> (String, TextureManager,  i32,) {

    
     let btn_exit = TextButton::new(0.0, 0.0, 50.0, 50.0, "X", RED, BLACK, 50);

    
    
    
    
    
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
        draw_text("Your objective is to destroy as many alien ships as you can", 100.0, 100.0, 30.0, WHITE);
        draw_text("press space to shoot bullets!", 300.0, 200.0, 30.0, WHITE);
         draw_text("if the aliens reach you, GAME OVER", 300.0, 300.0, 30.0, WHITE);
 
  draw_text("Press R For The Main Menu! ", 200.0, 500.0, 30.0, WHITE);
 
 if btn_exit.click() {
            return ("main".to_string(), tm, h_score);
        }

        if is_key_pressed(KeyCode::R) {
 return ("start".to_string(), tm, h_score);
        }

        next_frame().await;
    }
}
