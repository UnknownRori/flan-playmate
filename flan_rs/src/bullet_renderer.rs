use godot::{
    classes::{IMultiMeshInstance2D, MultiMeshInstance2D, canvas_item::TextureFilter},
    prelude::*,
};

use crate::bullet_manager::BulletManager;

#[derive(GodotClass)]
#[class(init, base=MultiMeshInstance2D)]
pub struct BulletRenderer {
    #[export]
    bullet_manager: Option<Gd<BulletManager>>,
    base: Base<MultiMeshInstance2D>,
}

#[godot_api]
impl IMultiMeshInstance2D for BulletRenderer {
    fn ready(&mut self) {
        let mut multimesh = self.base_mut().get_multimesh().unwrap();
        let manager_ptr = self.bullet_manager.clone().unwrap();
        let manager = manager_ptr.bind();
        multimesh.set_instance_count(manager.get_pool_size() as i32);
    }

    fn process(&mut self, _dt: f64) {
        let mut multimesh = self.base_mut().get_multimesh().unwrap();
        let texture = self.base().get_texture().unwrap();
        let manager_ptr = self.bullet_manager.clone().unwrap();
        let manager = manager_ptr.bind();
        let bullets = manager.get_bullets();

        let mut index = 0;
        for bullet in bullets {
            if !bullet.active {
                continue;
            }

            let mut transform = Transform2D::IDENTITY;
            transform.origin = bullet.position;

            let uv_offset = bullet.texture.position / texture.get_size();
            let uv_scale = bullet.texture.size / texture.get_size();

            multimesh.set_instance_transform_2d(index, transform);
            multimesh.set_instance_custom_data(
                index,
                Color::from_rgba(uv_offset.x, uv_offset.y, uv_scale.x, uv_scale.y),
            );

            index += 1;
        }

        multimesh.set_visible_instance_count(index);
    }
}
