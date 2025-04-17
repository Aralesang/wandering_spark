use bevy::prelude::*;
mod animation_sprite;
mod equipment;
mod player;
mod role;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(player::PlayerPlugin)
        .add_plugins(animation_sprite::AnimateSpritePlugin)
        .add_plugins(equipment::EquipmentPlugin)
        .run();
}