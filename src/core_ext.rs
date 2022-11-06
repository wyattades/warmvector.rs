use bevy::prelude::Vec2;

trait AngleExt {
    fn vec_angle(&self) -> f32;
}

impl AngleExt for Vec2 {
    fn vec_angle(&self) -> f32 {
        self.y.atan2(self.x)
    }
}
