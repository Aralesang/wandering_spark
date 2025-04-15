use bevy::prelude::*;

/**
 * 玩家
 */
#[derive(Component)]
struct Player{
    moveing: bool
}

/**
 * 名称
 */
#[derive(Component)]
struct Name(String);

/**
 * 帧动画
 */
#[derive(Component)]
struct AnimationIndices {
    size: usize,
    colum: usize,
    row: usize,
    direction: usize,
}

/**
 * 动画计时器
 */
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup.chain())
        .add_systems(Update, (move_player, animate_sprite))
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    //加载图片
    let texture: Handle<Image> = asset_server.load("image/anim/player/idle.png");
    //构建帧动画结构
    let indices = AnimationIndices {
        size: 32,
        colum: 1,
        row: 4,
        direction: 0,
    };
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
        Player{moveing: false},
        Transform {
            scale: Vec3::splat(1.0),
            ..default()
        },
        Name("露比".to_string()),
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
    mut transform: Single<&mut Transform, With<Player>>,
    mut sprite: Single<&mut Sprite, With<Player>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut indices:Single<&mut AnimationIndices, With<Player>>,
    mut player:Single<&mut Player, With<Player>>,
) {
    let mut x: f32 = 0.0;
    let mut y: f32 = 0.0;
    if keyboard_input.pressed(KeyCode::ArrowRight) {
        x += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        x -= 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowUp) {
        y += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) {
        y -= 1.0;
        if player.moveing == false {
            player.moveing = true;
            //修改动画
            sprite.image = asset_server.load("image/anim/player/walk.png");
            //构建帧动画结构
            indices.colum = 8;
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
                atlas.index = 0;
            }
        }
    }
    transform.translation.x += x;
    transform.translation.y += y;
}

/**
 * 精灵动画
 */
fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut Sprite, &mut AnimationTimer)>,
) {
    for (indices, mut sprite, mut timer) in &mut query {
        timer.tick(time.delta());
        //定时器控制播放速度
        if timer.finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == indices.direction * indices.row + indices.colum - 1 {
                    atlas.index = indices.direction * indices.row;
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}
