use bevy::prelude::*;

pub struct AnimateSpritePlugin;

impl Plugin for AnimateSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update);
    }
}

/**
 * 帧动画
 */
#[derive(Component)]
pub struct AnimationIndices {
    pub size: usize,
    pub colum: usize,
    pub row: usize,
    pub direction: usize,
}

/**
 * 动画计时器
 */
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

/**
 * 精灵动画
 */
fn update(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut Sprite, &mut AnimationTimer), With<AnimationIndices>>,
) {
    for (indices, mut sprite, mut timer) in &mut query {
        timer.tick(time.delta());
        //定时器控制播放速度
        if timer.finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == indices.direction * indices.colum + indices.colum - 1 {
                    atlas.index = indices.direction * indices.colum;
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}