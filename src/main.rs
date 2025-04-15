use bevy::{input::keyboard, prelude::*};

/**
 * 玩家
 */
#[derive(Component)]
struct Player {
    moveing: bool,
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
    mut indices: Single<&mut AnimationIndices, With<Player>>,
    mut player: Single<&mut Player, With<Player>>,
) {
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
        let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(layout);
        if let Some(atlas) = &mut sprite.texture_atlas {
            atlas.layout = texture_atlas_layout;
            atlas.index = indices.direction * indices.colum;
        }
    }
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
                if atlas.index == indices.direction * indices.colum + indices.colum - 1 {
                    atlas.index = indices.direction * indices.colum;
                } else {
                    atlas.index += 1;
                }
            }
        }
    }
}
