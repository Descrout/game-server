use specs::{Component, VecStorage};

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct IDComp {
    pub id: u32,
}

#[derive(Component, Debug, Clone)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32
}

#[derive(Component, Debug, Clone, Default)]
#[storage(VecStorage)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32
}

#[derive(Component, Debug, Clone, Default)]
#[storage(VecStorage)]
pub struct InputComp {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}