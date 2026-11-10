use crate::ui::still_image::{self, StillImage};
use macroquad::input::KeyCode;
use macroquad::prelude::*;

pub struct Player {
    image: StillImage,
    pub speed: f32,
    pub move_dir: Vec2,
    pub old_pos: Vec2,
    pub health: f32,
}

impl Player {
    pub async fn new(
        image: &str,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        stretch: bool,
        zoom: f32,
        speed: f32,
        health: f32,
    ) -> Self {
        let image = StillImage::new(image, width, height, x, y, stretch, zoom).await;
  

        Player {
            image,
            speed: speed,
            move_dir: Vec2::new(0.0, 0.0),
            old_pos: Vec2::new(0.0, 0.0),
            health: health,
        }
    }

  

    pub fn image(&self) -> &StillImage {
        &self.image
    }

       pub fn get_speed(&self) -> f32 {
        self.speed
    }

    pub fn keypress(&mut self) {
        self.move_dir = vec2(0.0, 0.0);
        self.old_pos = vec2(self.image.get_x(), self.image.get_y());
        // Keyboard input

        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            self.move_dir.x += self.speed;
            self.image.set_angle(0.15);
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            self.move_dir.x -= self.speed;
            self.image.set_angle(-0.15);
        }
        // if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
        //     self.move_dir.y += self.speed;
        // }
        // if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
        //     self.move_dir.y -= self.speed;
        // }

        if self.move_dir == vec2(0.0, 0.0) {
            self.image.set_angle(0.0);
        }
       

        if self.move_dir.length() > 0.0 {
            self.move_dir = self.move_dir.normalize() * 3.0;

            if self.move_dir.length() > self.speed {
                self.move_dir = self.move_dir.normalize() * self.speed;
            }
        }
    }

    pub fn draw(&self) {
        self.image.draw();
    }

    pub fn move_player(&mut self) {
        self.image.set_x(self.image.get_x() + self.move_dir.x);
        self.image.set_y(self.image.get_y() + self.move_dir.y);
    }
    pub fn move_back_x(&mut self) {
        self.image.set_x(self.old_pos.x);
    }
    pub fn move_back_y(&mut self) {
        self.image.set_y(self.old_pos.y);
    }

    pub fn get_health(&self) -> f32 {
        self.health
    }
    pub fn set_health(&mut self, health: f32) {
        self.health = health;
    }
}
