use bevy::prelude::*;
use crate::role::Role;
/**
 * 装备
 */
#[derive(Component, Deref, DerefMut)]
pub struct Equipment(pub String);

pub struct EquipmentPlugin;

impl Plugin for EquipmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, equipment_animation_sync);
    }
}

/** 装备动画同步 */
fn equipment_animation_sync(
    mut query: Query<(&Parent, &mut Sprite), (With<Equipment>, Without<Role>)>,
    body: Query<&Sprite, With<Role>>,
) {
    for (parent, mut equipment_sprite) in &mut query {
        if let Ok(body_sprite) = body.get(parent.get()) {
            let body_atlas = &body_sprite.texture_atlas.as_ref().unwrap();
            // 最后处理 texture_atlas
            let equipment_atlas: &mut &mut TextureAtlas = &mut equipment_sprite.texture_atlas.as_mut().unwrap();
            equipment_atlas.index = body_atlas.index;
        }
    }
}