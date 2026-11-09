use crate::ui::still_image::{self, StillImage};
use macroquad::input::KeyCode;
use macroquad::prelude::*;

pub struct enemy {
    image: StillImage,
    pub speed: f32,
    pub move_dir: Vec2,
    pub min_y: f32,
    pub max_y: f32,
}

impl enemy {
    pub async fn new(
        image: &str,
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        stretch: bool,
        zoom: f32,
        speed: f32,
    ) -> Self {
        let image = StillImage::new(image, width, height, x, y, stretch, zoom).await;
  

        enemy {
            image,
            speed: speed,
            move_dir: Vec2::new(0.0, 0.0),
            min_y: 0.0,
            max_y: screen_height(),
        }
    }

  
pub fn slide(&mut self) {
        self.move_dir = vec2(0.0, 1.0);

        if self.move_dir.length() > 0.0 {
            self.image.set_y(
                self.image.get_y() + self.move_dir.y * self.speed, //* get_frame_time() * 60.0
            );
        }
    }

    pub fn get_image(&self) -> &StillImage {
        &self.image
    }

       pub fn get_speed(&self) -> f32 {
        self.speed
    }
    
    pub fn draw(&self) {
        self.image.draw();
    }

  
}
