use bevy::prelude::*;

#[derive(Component)]
struct Player();

#[derive(Component)]
struct Name(String);

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .add_systems(Startup, (setup, show_name).chain())
    .run();
}

fn setup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>) {
    let texture:Handle<Image> = asset_server.load("image/anim/player/idle.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(32), 1, 4, None, None);
    let texture_atlas_layout: Handle<TextureAtlasLayout> = texture_atlas_layouts.add(layout);
    commands.spawn(Camera2d);
    commands.spawn((
        Player(),
        Name("露比".to_string()),
        Sprite::from_atlas_image(texture, TextureAtlas{
            layout: texture_atlas_layout,
            index: 0
        }),
        Transform::from_scale(Vec3::splat(6.0))
    ));
}

fn show_name(query: Query<&Name>){
    for name in &query {
        println!("{}", name.0)
    }
}