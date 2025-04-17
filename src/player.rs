use bevy::prelude::*;

use crate::animation_sprite::*;
use crate::equipment::*;
use crate::role::Role;

/**
 * 玩家
 */
#[derive(Component)]
pub struct Player;

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
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);
    spawn_player(commands, asset_server, texture_atlas_layouts);
}

/**
 * 创建玩家实体
 */
fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    //构建帧动画结构
    let player_indices = AnimationIndices {
        size: 32,
        colum: 1,
        row: 4,
        direction: 0,
    };
    //加载图片
    let player_texture: Handle<Image> = asset_server.load("image/anim/player/idle.png");

    //构建纹理布局
    let player_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(player_indices.size as u32),
        player_indices.colum as u32,
        player_indices.row as u32,
        None,
        None,
    );
    //将纹理布局添加到处理器
    let player_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(player_layout);
    let player_entity = commands
        .spawn((
            Player,
            Role {
                moveing: false,
                action: "idle".to_string(),
            },
            Transform {
                scale: Vec3::splat(3.0),
                ..default()
            },
            Sprite::from_atlas_image(
                player_texture,
                TextureAtlas {
                    layout: player_texture_atlas_layout,
                    index: 0,
                },
            ),
            player_indices,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        ))
        .id();

    //加载图片
    let equpment_texture: Handle<Image> = asset_server.load("image/anim/work_clothe/idle.png");
    //构建帧动画结构
    let equipment_indices = AnimationIndices {
        size: 32,
        colum: 1,
        row: 4,
        direction: 0,
    };
    //构建纹理布局
    let equpment_layout = TextureAtlasLayout::from_grid(
        UVec2::splat(equipment_indices.size as u32),
        equipment_indices.colum as u32,
        equipment_indices.row as u32,
        None,
        None,
    );
    //将纹理布局添加到处理器
    let equpment_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(equpment_layout);

    //构建装备实体，并作为玩家的子实体
    commands.entity(player_entity).with_children(|praent| {
        praent.spawn((
            Equipment("work_clothe".to_string()),
            Sprite::from_atlas_image(
                equpment_texture,
                TextureAtlas {
                    layout: equpment_texture_atlas_layout,
                    index: 0,
                },
            ),
            equipment_indices,
        ));
    });
}

/**
 * 玩家移动控制
 */
fn move_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut query: Query<(
        &mut Transform, &mut AnimationIndices, &mut Role,
        &mut Sprite
    ), With<Player>>,
) {
    for (
        mut transform, mut indices, mut role,
        mut sprite,
    ) in &mut query {
        let mut x: f32 = 0.0;
        let mut y: f32 = 0.0;
        let mut direction: usize = indices.direction;
        let mut move_ing: bool = false;
        let mut colum: usize = indices.colum;

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
        if !move_ing && role.moveing {
            role.moveing = false;
            role.action = "idle".to_string();
            set_player_img(
                sprite, asset_server, 
                texture_atlas_layouts, 
                indices, "player".to_string(), 
                "idle".to_string(), direction
            );
        }

        //进入移动
        if move_ing && !role.moveing {
            role.moveing = true;
            role.action = "walk".to_string();
        }

        if direction != indices.direction {
            indices.colum = colum;
            indices.direction = direction;
        }
    }
}

/** 设置玩家图像 */
fn set_player_img(
    mut sprite: Mut<Sprite>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut indices: Mut<AnimationIndices>,
    animation_name: String,
    animation_action: String,
    direction: usize,
) {
    let path = format!("image/anim/{}/{}.png", animation_name, animation_action);
    //修改动画
    sprite.image = asset_server.load(path);
    //构建帧动画结构
    if animation_action == "idle" {
        indices.colum = 1;
    } else if animation_action == "walk" {
        indices.colum = 8;
    }
    indices.direction = direction;
    //构建纹理布局
    let layout = TextureAtlasLayout::from_grid(
        UVec2::splat(indices.size as u32),
        indices.colum as u32,
        indices.row as u32,
        None,
        None,
    );
    let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(layout);
    if let Some(atlas) = &mut sprite.texture_atlas {
        atlas.layout = texture_atlas_layout;
        atlas.index = indices.direction * indices.colum;
    }
}
