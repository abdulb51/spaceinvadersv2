use macroquad::prelude::*;

use crate::custom::player::Player;
use crate::custom::projectile::projectile;
use crate::ui::text_button::TextButton;

pub async fn run() -> String{
    let btn_exit = TextButton::new(0.0, 0.0, 50.0, 50.0, "X", RED, BLACK, 50);

    let mut player = Player::new("assets/ship.png",
      55.0,
       405.0,
        50.0,
         50.0,
          true,
           1.0,
            1.0,
             5.0).await;

  
let mut bullets_live: Vec<projectile> = Vec::new();

let mut cooldown = 0.0;
    
    
    loop {
        clear_background(WHITE);
        draw_text("game", 20.0, 40.0, 30.0, WHITE);

        player.keypress();
        player.move_player();



if is_key_pressed(KeyCode::Space)&& cooldown == 0.0 {
 get_frame_time();
     let mut bullet: projectile = projectile::new(
        player.get_image().get_x() + 25.0, 
        player.get_image().get_y(),
         100.0,
          100.0, 
          "assets/bullet.png",
           true,
            1.0,
             0.0).await;
             bullet.shoot();
             cooldown = 0.1;
}

        if is_key_pressed(KeyCode::R) {
            return "win".to_string();
        }

        if btn_exit.click() {
            return "main".to_string();
        }

        player.get_image().draw();
        next_frame().await;



//println!("screenhight {}",screen_height());
// println!("screenwidth {}",screen_width());
    }
}
