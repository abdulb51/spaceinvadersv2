/*
By: <Abdul Baig>
Date: 2026-10-14
Program Details: <Space Invaders Game>
*/

mod ui;
mod utils;
mod custom;

mod start;
mod game;
mod win;
mod lose;



use macroquad::prelude::*;
use crate::utils::preload_image::TextureManager;
use crate::ui::animated_image::AnimatedImage;
/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "Space Invaders".to_string(),
        window_width: 920,
        window_height:920,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}


#[macroquad::main(window_conf)]
async fn main() {
    
    
    let mut tm = TextureManager::new();

 let all_files = vec! [
    "assets/ship.png",
  "assets/shipboost.png",
   "assets/evil.png",
   "assets/evilb.png", 
   "assets/bullet.png",
    "assets/bg.gif"
   ];
   tm.preload_with_loading_screen(&all_files, None, None).await;


//   tm.preload_with_loading_screen(&
    // ["assets/ship.png",
//   "assets/shipboost.png",
//    "assets/evil.png",
//    "assets/evilb.png", 
//    "assets/bullet.png",
//    "assets/bg.gif"
//    ],
    // None, None).await;

    let mut s_score = 0;
    let mut h_score = 0;

    
    let mut current_screen = "start".to_string();
    let mut last_switch = get_time() - 0.02;

 

    loop {
        if get_time() - last_switch > 0.01 {
           (current_screen, tm, s_score, h_score)  = match current_screen.as_str() {
                "start" => start::run(tm, s_score, h_score).await,
                "game" => game::run(tm, s_score, h_score).await,
                "win" => win::run(tm, s_score, h_score).await,
                "lose" => lose::run(tm, s_score, h_score).await,
                _ => break,
            };
            last_switch = get_time();




        }
        next_frame().await;
    }
}