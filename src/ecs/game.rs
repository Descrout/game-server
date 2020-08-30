use crate::proto::proto_all::State;
use specs::prelude::*;
use super::components::*;
use std::collections::HashMap;

pub struct Game{
    world: World,
    players: HashMap<u32, Entity>,
    pub state: State,
}

impl Game {
    pub fn new(admin_id: u32) -> Self {
        let mut game = Self{
            world: Self::setup(),
            players: HashMap::new(),
            state: State{entities:Vec::new()},
        };
        game.add_player(admin_id);
        game
    }

    fn setup() -> World {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<InputComp>();
        world.register::<IDComp>();
        world
    }


    pub fn add_player(&mut self, id: u32) {
        let ent = self.world.create_entity()
        .with(Position{x: 100.0, y: 100.0})
        .with(IDComp{id})
        .with(Velocity::default())
        .with(InputComp::default())
        .build();
        self.players.insert(id, ent);
    }

    pub fn remove_player(&mut self, id: &u32) {
        let ent = self.players.remove(id).unwrap();
        let _ = self.world.delete_entity(ent);
    }

    pub fn set_input(&mut self, id: u32, input: crate::proto::proto_all::GameInput) {
        let ent = *self.players.get(&id).unwrap();
        let mut input_comp = self.world.write_storage::<InputComp>();
        let inpt = input_comp.get_mut(ent).unwrap();
        inpt.up = input.up;
        inpt.down = input.down;
        inpt.left = input.left;
        inpt.right = input.right;

    }

    pub fn update(&mut self) {
        let mut sys = InputSystem;
        sys.run_now(&self.world);

        let mut sys = VelSystem;
        sys.run_now(&self.world);

        self.world.maintain();

        self.state.entities.clear();
        let id = self.world.read_storage::<IDComp>();
        let pos = self.world.read_storage::<Position>();
        for(id, pos) in (&id, &pos).join() {
            self.state.entities.push(crate::proto::proto_all::Entity{id: id.id, x: pos.x, y: pos.y});
        }
    } 
}

pub struct VelSystem;

impl<'a> System<'a> for VelSystem {
    type SystemData = (WriteStorage<'a, Position>,
                    ReadStorage<'a, Velocity>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut pos, vel) = data;
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.x += vel.dx;
            pos.y += vel.dy;
        }
    
    }
}

pub struct InputSystem;

impl<'a> System<'a> for InputSystem {
    type SystemData = (ReadStorage<'a, InputComp>,
                    WriteStorage<'a, Velocity>);

    fn run(&mut self, data: Self::SystemData) {
        let (input, mut vel) = data;
        for (input, vel) in (&input, &mut vel).join() {
            vel.dx = 0.0;
            vel.dy = 0.0;
            if input.up {
                vel.dy = -10.0;
            }
            if input.down {
                vel.dy = 10.0;
            }
            if input.left {
                vel.dx = -10.0;
            }
            if input.right {
                vel.dx = 10.0;
            }
        }
    
    }
}