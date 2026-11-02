use crate::ui::still_image::{self, StillImage};
use macroquad::input::KeyCode;
use macroquad::prelude::*;

pub struct enemy {
    image: StillImage,
    pub speed: f32,
    pub move_dir: Vec2,
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
