use bevy::prelude::Vec2;
use geo::Rect;
use rand::prelude::*;

pub trait AngleExt {
    fn vec_angle(&self) -> f32;
}

impl AngleExt for Vec2 {
    fn vec_angle(&self) -> f32 {
        self.y.atan2(self.x)
    }
}

pub trait RandRectPoint {
    fn rand_rect_point(&mut self, rect: &Rect<f32>) -> Vec2;
}
impl RandRectPoint for ThreadRng {
    fn rand_rect_point(&mut self, rect: &Rect<f32>) -> Vec2 {
        Vec2::new(
            self.gen_range(rect.min().x..rect.max().x),
            self.gen_range(rect.min().y..rect.max().y),
        )
    }
}

pub trait RectExt {
    fn expand(&self, amount: f32) -> Rect<f32>;
}
impl RectExt for Rect<f32> {
    fn expand(&self, amount: f32) -> Rect<f32> {
        let clone = self.clone();
        clone.min().x -= amount;
        clone.max().x += amount;
        clone.min().y -= amount;
        clone.max().y += amount;
        clone
    }
}
