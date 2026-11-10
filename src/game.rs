use macroquad::input::KeyCode::X;
use macroquad::prelude::*;

use crate::custom::enemy::enemy;
use crate::custom::player::Player;
use crate::custom::projectile::projectile;
use crate::ui::still_image::StillImage;
use crate::ui::text_button::TextButton;
use crate::ui::grid::draw_grid;
use crate::utils::collision::check_collision;

pub async fn run() -> String {
    let btn_exit = TextButton::new(0.0, 0.0, 
        50.0, 50.0,
        "X", RED, BLACK, 50);

    let mut player = Player::new("assets/ship.png", 290.0, 800.0,
     100.0, 100.0,
      true, 1.0,
       1.0, 5.0).await;

    let mut bullets: Vec<projectile> = Vec::new();

    let mut enemies: Vec<enemy> = Vec::new();




let columns = 5;
let rows = 3;
let space_x=80.0;
let space_y = 80.0;

for row in 0..rows {
    for column in 0..columns {
        let enemy = enemy::new(
            "assets/evil.png",
            space_x * column as f32,
            space_y * row as f32,
            65.0,
            65.0,
            true,
            1.0,
            1.0,
            0.0,
            screen_height()/1.25,
            0.0,
            0.0,
        ).await;
        enemies.push(enemy);
        
}
}


    let mut cooldown = 0.0;

    loop {
        clear_background(WHITE);
    draw_grid(50.0, BLACK);


    


    for enemy in &mut enemies {
            enemy.image().draw();
        }

        // for enemy_index in (0..enemies.len()).rev() {
        //    let enemy = &mut enemies[enemy_index];
             
        //      {
        //         enemies.remove(enemy_index);
        //         continue;
        //     }
            
        // }
        player.keypress();
        player.move_player();

        if is_key_pressed(KeyCode::Space) && cooldown <= 0.0 {
            get_frame_time();
            
            
            let mut bullet: projectile = projectile::new(
                player.image().get_x(),
                player.image().get_y(),
                20.0,
                20.0,
                "assets/bullet.png",
                true,
                1.0,
                1.0,
            ).await;
           
           bullet.shoot();
            cooldown = 1.0;

            bullets.push(bullet);
        }

        if cooldown > 0.0 {
            cooldown -= get_frame_time();
        }

        for bullet_index in (0..bullets.len()).rev() {
            let bullet = &mut bullets[bullet_index];

              
                bullet.shoot();
                bullet.image().draw();
            
                if bullet.image().get_y() < -900.0 {
                bullet.image.clear();
                bullets.remove(bullet_index);
                continue;
            }
        }

        let collision = check_collision(bullets[0].image(), enemies[0].image(), 1); //Where 1 is the number of pixels to skip
    if collision {
        bullets[0].image().clear();
        enemies[0].image().clear();
    } else {
        continue;
    }

        if is_key_pressed(KeyCode::R) {
            return "start".to_string();
        }

        if btn_exit.click() {
            return "main".to_string();
        }
        
        if enemies.len() == 0 {
    return "win".to_string();
}

        player.image().draw();
        next_frame().await;
        
        // println!("screenhight {}",screen_height());
        // println!("screenwidth {}",screen_width());
    }
}
