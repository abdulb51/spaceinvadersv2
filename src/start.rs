

use image::codecs::pnm::ArbitraryTuplType::BlackAndWhiteAlpha;
use macroquad::prelude::*;
use crate::utils::preload_image::TextureManager;
use crate::ui::animated_image::AnimatedImage;
use crate::ui::still_image::StillImage;
use crate::ui::grid::draw_grid;
use crate::ui::text_button::TextButton;


   



pub async fn run(tm: TextureManager, h_score: i32) -> (String, TextureManager, i32,) {
  
    let bg = StillImage::from_preload(
        tm.get_preload("assets/bg2.png").unwrap(),
        920.0,
        920.0,
        0.0,
        0.0,
        true,
        1.0,
    );
 let title = StillImage::from_preload(
     tm.get_preload("assets/start.png").unwrap(),
        900.0,
        800.0,
        0.0,
        0.0,
        true,
        1.0,
    );

 let btn_play = TextButton::new(
        360.0,
        500.0,
        200.0,
        60.0,
        "Play",
        BLACK,
        LIGHTGRAY,
        70
    );
    
     let btn_guide = TextButton::new(
        360.0,
        580.0,
        200.0,
        60.0,
        "Guide",
        BLACK,
        LIGHTGRAY,
        70
    );
    
     let btn_themesandsuch = TextButton::new(
        360.0,
        200.0,
        200.0,
        60.0,
        "themes and such",
        BLUE,
        GREEN,
        30
    );
    
  





  loop {
      clear_background(WHITE);
      bg.draw();
      title.draw();
        draw_text("Screen 2", 20.0, 40.0, 30.0, WHITE);

       

if btn_guide.click()
{
    return ("guide".to_string(), tm, h_score);
}
if btn_play.click()
{
    return ("game".to_string(), tm, h_score);
}



 
 draw_grid(50.0, WHITE);
        next_frame().await;
    }
}











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