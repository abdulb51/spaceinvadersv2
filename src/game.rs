use macroquad::input::KeyCode::X;
use macroquad::prelude::*;

use crate::custom::enemy::enemy;
use crate::custom::player::Player;
use crate::custom::projectile::projectile;
use crate::ui::grid::draw_grid;
use crate::ui::text_button::TextButton;
use crate::utils::collision::check_collision;
use crate::utils::preload_image::TextureManager;
use crate::ui::animated_image::AnimatedImage;
use crate::ui::still_image::StillImage;

pub async fn run(tm: TextureManager, s_score: i32, h_score: i32) -> (String, TextureManager, i32, i32) {
    let btn_exit = TextButton::new(0.0, 0.0, 50.0, 50.0, "X", RED, BLACK, 50);

    let mut player = Player::new(
        "assets/ship.png",
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




    let mut bullets: Vec<projectile> = Vec::new();

    let mut enemies: Vec<enemy> = Vec::new();

    let columns = 5;
    let rows = 3;
    let space_x = 80.0;
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
                // 0.0,
                // screen_height() / 1.25,
                // 0.0,
                // 750.0,
            )
            .await;
            enemies.push(enemy);
        }
    }

    let mut min_x = 0.0;
    let mut max_x = 750.0;
    let mut max_y = screen_height() / 1.25;

    let mut cooldown = 0.0;

    loop {
        clear_background(WHITE);
        draw_grid(50.0, BLACK);

        for enemy in 0..enemies.len() {
            if enemies[enemy].image().get_x() <= max_x {
                let move_dir = vec2(1.0, 0.0);
                println!("max_x: {}", max_x);
                let temp = enemies[enemy].image().get_x() + move_dir.x * enemies[enemy].get_speed();
                enemies[enemy].set_x(temp);
            } else if enemies[enemy].image().get_x() == max_x {
                let max_x = max_x;
                let move_dir = vec2(-1.0, 0.0);
                println!("max_x: {}", max_x);
                let temp = enemies[enemy].image().get_x() + move_dir.x * enemies[enemy].get_speed();
                enemies[enemy].set_x(temp);
            }
        }

        for enemy in 0..enemies.len() {
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
                "assets/bullet.png",
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
            for blast in 0..bullets.len() {
                let collision = check_collision(bullets[0].image(), enemies[hit].image(), 1); //Where 1 is the number of pixels to skip
                if collision {
                    bullets[blast].clear();
                    enemies[hit].clear();
                }
            }
        }

        if is_key_pressed(KeyCode::R) {
            return ("start".to_string(), tm, s_score, h_score);
        }

        if btn_exit.click() {
            return ("main".to_string(), tm, s_score, h_score);
        }

        if enemies.len() == 0 {
            return ("win".to_string(), tm, s_score, h_score);
        }

        player.image().draw();
        next_frame().await;

        // println!("screenhight {}",screen_height());
        // println!("screenwidth {}",screen_width());
    }
}
