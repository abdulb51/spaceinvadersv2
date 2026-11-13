use macroquad::input::KeyCode::X;
use macroquad::prelude::*;

use crate::custom::enemy::enemy;
use crate::custom::player::Player;
use crate::custom::projectile::projectile;
use crate::ui::animated_image::AnimatedImage;
use crate::ui::grid::draw_grid;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::ui::text_button::TextButton;
use crate::utils::collision::check_collision;
use crate::utils::preload_image::TextureManager;

pub async fn run(tm: TextureManager, h_score: i32) -> (String, TextureManager, i32) {
    let btn_exit = TextButton::new(0.0, 0.0, 50.0, 50.0, "X", RED, BLACK, 50);


    let mut player = Player::new(
        tm.get_preload("assets/ship.png").unwrap(), //you ade asset not assets
        290.0,
        800.0,
        100.0,
        100.0,
        true,
        1.0,
        1.0,
        5.0,
    )
    .await;
   
   
    let mut lbl_s_score = Label::new("SCORE: 0 ", 50.0, 100.0, 30);
let mut s_score = 0;

    let mut bullets: Vec<projectile> = Vec::new();

    let mut enemies: Vec<enemy> = Vec::new();

    let columns = 5;
    let rows = 3;
    let space_x = 80.0;
    let space_y = 80.0;
    for row in 0..rows {
        for column in 0..columns {
            let enemy = enemy::new(
                tm.get_preload("assets/evil.png").unwrap(),
                space_x * column as f32,
                space_y * row as f32,
                65.0,
                65.0,
                true,
                1.0,
                1.0,
            )
            .await;
            enemies.push(enemy);
        }
    }

    let mut min_x = 0.0;
    let mut max_x = 850.0;
    let max_y = screen_height() / 1.25;

    let mut cooldown = 0.0;
    let mut move_dir = 1.0;

    loop {
        clear_background(WHITE);
        draw_grid(50.0, BLACK);

    //   if rows == 2 {
    //       min_x -= 65.0;
    //       max_x += 65.0;
    //   } 

        lbl_s_score.set_text("SCORE: ".to_string() + &s_score.to_string());
        lbl_s_score.draw();

        let mut jump = false;
        for enemy in 0..enemies.len() {
            if enemies[enemy].image().get_x() > max_x {
                jump = true;
                move_dir *= -1.0;
                break;
            }
            if enemies[enemy].image().get_x() < min_x {
                jump = true;

                move_dir *= -1.0;
                break;
            }
        }

        for enemy in 0..enemies.len() {
            let temp: f32 = enemies[enemy].image().get_x() + move_dir * enemies[enemy].get_speed();
            enemies[enemy].set_x(temp);
            if jump {
                let temp: f32 = enemies[enemy].image().get_y() + 30.0;

                enemies[enemy].set_y(temp);
            }

            enemies[enemy].image().draw();
        }

        player.keypress();
        player.move_player();

        if is_key_pressed(KeyCode::Space) && cooldown <= 0.0 {
            get_frame_time();

            let mut bullet: projectile = projectile::new(
                player.image().get_x(),
                player.image().get_y(),
                40.0,
                50.0,
                tm.get_preload("assets/bullet.png").unwrap(),
                true,
                1.0,
                1.0,
            )
            .await;

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

        for hit in 0..enemies.len() {
            for shoot in 0..bullets.len() {
                let collision = check_collision(bullets[0].image(), enemies[hit].image(), 1); //Where 1 is the number of pixels to skip
                if collision {
                    bullets[shoot].clear();
                    enemies[hit].clear();
                    s_score += 50;
                }
            }
        }

        if is_key_pressed(KeyCode::R) {
            return ("start".to_string(), tm, h_score);
        }

        if btn_exit.click() {
            return ("main".to_string(), tm, h_score);
        }

        if enemies.len() == 0 {
            return ("win".to_string(), tm, h_score);
        }

        player.image().draw();
        next_frame().await;

        // println!("screenhight {}",screen_height());
        // println!("screenwidth {}",screen_width());
    }
}
