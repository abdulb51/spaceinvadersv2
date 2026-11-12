

use macroquad::prelude::*;
use crate::utils::preload_image::TextureManager;
use crate::ui::animated_image::AnimatedImage;
use crate::ui::still_image::StillImage;



pub async fn run(tm: TextureManager, s_score: i32, h_score: i32) -> (String, TextureManager, i32, i32,) {
  
//   let mut bg = AnimatedImage::new(
//         "assets/bg.png", 
//         100.0, 100.0,                      
//         1920.0,
//          1080.0,                        
//         1, 1,                              
//         1.0,                               
//         true                               
//     ).await;
//  let preloaded = tm
//         .get_preloaded_animated_gif("assets/bg.gif")
//         .unwrap();
//  let mut gif_sprite_preloaded = AnimatedImage::from_preloaded_gif(
//         preloaded,

//         300.0,
//         100.0,
//         128.0,
//         128.0,
//         true,
//     );
// Or load directly from a GIF file (works on both web and native platforms)
    // let mut bg = AnimatedImage::from_gif(
    //      "assets/bg.gif", 
    //      0.0, 0.0,          
    //      1920.0, 1080.0,          
    //      true                   
    //  ).await;

  loop {
        clear_background(WHITE);
        draw_text("Screen 2", 20.0, 40.0, 30.0, WHITE);

        if is_key_pressed(KeyCode::Space) {
            return ("game".to_string(), tm, s_score, h_score);
        }
//  bg.draw();
        next_frame().await;
    }
}