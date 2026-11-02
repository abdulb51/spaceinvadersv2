use crate::custom::player;
use crate::ui::still_image::StillImage;
use macroquad::input::KeyCode;
use macroquad::prelude::*;

pub struct projectile {
   pub image: StillImage,
    pub move_dir: Vec2,
    pub speed: f32,
}

impl projectile {
    pub async fn new(x: f32, y: f32, width: f32, height: f32, bullet: &str, stretch: bool, zoom: f32, speed: f32) -> Self {
        let image = StillImage::new(bullet, width, height, x, y, stretch, 1.0).await;

        projectile {
            image: image,
            move_dir: Vec2::new(0.0, 0.0),
            speed: speed,
        }
    }

    pub fn draw(&self) {
        self.image.draw();
    }
    
    pub fn image(&self) -> &StillImage {
            &self.image
        }


    pub fn shoot(&mut self) {
        self.move_dir = vec2(0.0, -8.0);

        if self.move_dir.length() > 0.0 {
            self.image.set_y(
                self.image.get_y() + self.move_dir.y * self.speed, //* get_frame_time() * 60.0
            );
        }

        
    }
}
