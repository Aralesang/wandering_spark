use bevy::prelude::*;
use crate::animation_sprite::*;
use crate::role::Role;
/**
 * 装备
 */
#[derive(Component)]
pub struct Equipment;

pub struct EquipmentPlugin;

impl Plugin for EquipmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, equipment_animation_sync);
    }
}

/** 装备动画同步 */
fn equipment_animation_sync(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut query: Query<(&Parent, &mut Sprite), (With<Equipment>, Without<Role>)>,
    body: Query<(&Sprite, &AnimationIndices), With<Role>>,
) {
    for (parent, mut equipment_sprite) in &mut query {
        if let Ok((body_sprite, body_indices)) = body.get(parent.get()) {
            let body_atlas = &body_sprite.texture_atlas.as_ref().unwrap();
            let mut path = String::from("image/anim/work_clothe/");
            path.push_str(body_indices.name.clone().as_str());
            path.push_str(".png");
            equipment_sprite.image = asset_server.load(path.clone().as_str());
            //构建纹理布局
            let layout = TextureAtlasLayout::from_grid(
                UVec2::splat(body_indices.size as u32),
                body_indices.colum as u32,
                body_indices.row as u32,
                None,
                None,
            );
            let texture_atlas_layout: Handle<TextureAtlasLayout> =
                texture_atlas_layouts.add(layout);
            // 最后处理 texture_atlas
            let equipment_atlas = &mut equipment_sprite.texture_atlas.as_mut().unwrap();
            equipment_atlas.layout = texture_atlas_layout;
            equipment_atlas.index = body_atlas.index;
        }
    }
}
