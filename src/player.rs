use bevy::prelude::*;
use crate::animate_sprite::*;

/**
 * 玩家
 */
#[derive(Component)]
struct Player {
    moveing: bool,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, move_player);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    //构建帧动画结构
    let indices = AnimationIndices {
        size: 32,
        colum: 1,
        row: 4,
        direction: 0,
    };
    //加载图片
    let texture: Handle<Image> = asset_server.load("image/anim/player/idle.png");

    //构建纹理布局
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(indices.size as u32),
        indices.colum as u32,
        indices.row as u32,
        None,
        None,
    );
    //将纹理布局添加到处理器
    let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(layout);
    commands.spawn(Camera2d);
    commands.spawn((
        Player { moveing: false },
        Transform {
            scale: Vec3::splat(3.0),
            ..default()
        },
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
        ),
        indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
    ));
}

/**
 * 玩家移动控制
 */
fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut query: Query<(&mut Transform, &mut Sprite, &mut AnimationIndices, &mut Player)>,
) {
    for (mut transform, mut sprite, mut indices, mut player) in &mut query {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut direction: usize = indices.direction;
        let mut move_ing: bool = false;
        let mut path: &str = "";
        let mut colum: usize = indices.colum;
        //是否改变图像
        let mut change: bool = false;

        if keyboard_input.pressed(KeyCode::ArrowRight) {
            x += 1.0;
            direction = 2;
        }
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            x -= 1.0;
            direction = 3;
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) {
            y += 1.0;
            direction = 1;
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) {
            y -= 1.0;
            direction = 0;
        }

        if y != 0.0 || x != 0.0 {
            move_ing = true;
            transform.translation.x += x;
            transform.translation.y += y;
        }

        //进入闲置
        if !move_ing && player.moveing {
            path = "image/anim/player/idle.png";
            colum = 1;
            player.moveing = false;
            change = true;
        }

        //进入移动
        if move_ing && !player.moveing {
            path = "image/anim/player/walk.png";
            colum = 8;
            player.moveing = true;
            change = true;
        }

        if direction != indices.direction {
            change = true;
        }

        //如果需要改变图像
        if change {
            //修改动画
            if path != "" {
                sprite.image = asset_server.load(path);
            }
            //构建帧动画结构
            indices.colum = colum;
            indices.direction = direction;
            //构建纹理布局
            let layout = TextureAtlasLayout::from_grid(
                UVec2::splat(indices.size as u32),
                indices.colum as u32,
                indices.row as u32,
                None,
                None,
            );
            let texture_atlas_layout: Handle<TextureAtlasLayout> =
                texture_atlas_layouts.add(layout);
            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.layout = texture_atlas_layout;
                atlas.index = indices.direction * indices.colum;
            }
        }
    }
}


