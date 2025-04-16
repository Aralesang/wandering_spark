use bevy::prelude::*;
mod animate_sprite;
mod player;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(player::PlayerPlugin)
        .add_plugins(animate_sprite::AnimateSpritePlugin)
        .run();
}