use super::components::*;
use specs::prelude::*;

pub struct VelSystem;

impl<'a> System<'a> for VelSystem {
    type SystemData = (WriteStorage<'a, Position>, WriteStorage<'a, Velocity>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut pos, mut vel) = data;
        for (pos, vel) in (&mut pos, &mut vel).join() {
            pos.x += vel.dx;
            pos.y += vel.dy;
            vel.dx = 0.0;
            vel.dy = 0.0;
        }
    }
}
