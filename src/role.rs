use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Role{
    /** 是否正在移动 */
    pub moveing: bool, 
    /** 当前的动作 */
    pub action: String,
}