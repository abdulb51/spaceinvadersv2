use crate::ui::still_image::{self, StillImage};
use macroquad::input::KeyCode;
use macroquad::prelude::*;

pub struct enemy {
    image: StillImage,
    pub speed: f32,
    pub move_dir: Vec2,
    pub min_y: f32,
    pub max_y: f32,
    pub min_x : f32,
    pub max_x : f32
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
        min_y: f32,
        max_y: f32,
        min_x : f32,
        max_x : f32
    ) -> Self {
        let image = StillImage::new(image, width, height, x, y, stretch, zoom,).await;
  

        enemy {
            image,
            speed: speed,
            move_dir: Vec2::new(0.0, 0.0),
            min_y: min_y,
            max_y: max_y,
            min_x: min_x,
            max_x: max_x
        }
    }

  
pub fn slide(&mut self) {
        self.move_dir = vec2(1.0, 0.0);
    }

    pub fn slide_down(&mut self) {
        self.move_dir = vec2(0.0, 1.0);
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
