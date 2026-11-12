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
        let image = StillImage::new(image, width, height, x, y, stretch, zoom,).await;
  

        enemy {
            image,
            speed: speed,
            move_dir: Vec2::new(0.0, 0.0),
      
        }
    }

  

    pub fn slide_down(&mut self) {
        self.move_dir = vec2(0.0, 10.0);
    }

    pub fn image(&self) -> &StillImage {
        &self.image
    }
pub fn set_x(&mut self, x: f32) {
        self.image.set_x(x);
    }
       pub fn get_speed(&self) -> f32 {
        self.speed
    }
    
    pub fn draw(&self) {
        self.image.draw();
    }

    pub fn clear(&mut self) {
        self.image.clear();
    }


//     pub fn get_min_y(&self) -> f32 {
//         self.min_y
//     }
//     pub fn get_max_y(&self) -> f32 {
//         self.max_y
//     }
//     pub fn get_min_x(&self) -> f32 {
//        self.min_x
// }
// pub fn get_max_x(&self) -> f32 {
//         self.max_x
//     }
}