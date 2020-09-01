use specs::prelude::*;
use super::components::*;
use std::collections::HashMap;
use super::systems::*;
use super::game_state::GameState;



pub struct Game{
    world: World,
    players: HashMap<u32, Entity>,
    pub states: Vec<GameState>,
}

impl Game {
    pub fn new(admin_id: u32) -> Self {
        let mut game = Self{
            world: Self::setup(),
            players: HashMap::new(),
            states: Vec::new(),
        };
        game.add_player(admin_id);
        game
    }

    fn setup() -> World {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<IDComp>();
        world
    }


    pub fn add_player(&mut self, id: u32) {
        let ent = self.world.create_entity()
        .with(Position{x: 100.0, y: 100.0, angle: 0.0, last_seq: 0})
        .with(IDComp{id})
        .with(Velocity::default())
        .build();
        self.players.insert(id, ent);
        self.states.push(GameState::new(id));
    }

    pub fn remove_player(&mut self, id: &u32) {
        let ent = self.players.remove(id).unwrap();
        let _ = self.world.delete_entity(ent);
        self.states.retain(|gs| gs.id != *id);
    }

    pub fn set_input(&mut self, id: u32, input: crate::proto::proto_all::GameInput) {
        let ent = *self.players.get(&id).unwrap();
        //let mut vel = self.world.write_storage::<Velocity>();
        let mut pos = self.world.write_storage::<Position>();
        //let vel = vel.get_mut(ent).unwrap();
        let pos = pos.get_mut(ent).unwrap();
        if input.up {
            pos.y -= 300.0 * crate::DT;
        }
        if input.down {
            pos.y += 300.0 * crate::DT;
        }
        if input.left {
            pos.x -= 300.0 * crate::DT;
        }
        if input.right {
            pos.x += 300.0 * crate::DT;
        }
        pos.angle = input.angle;
        pos.last_seq = input.sequence;
    }

    pub fn update(&mut self) {
        //let mut sys = VelSystem;
        //sys.run_now(&self.world);

        self.world.maintain();

        let player = self.world.read_storage::<IDComp>();
        let pos = self.world.read_storage::<Position>();

        for gs in self.states.iter_mut(){
            gs.clear();
            for(player, pos) in (&player, &pos).join() {
                gs.state.last_seq = pos.last_seq;
                gs.add_entity(crate::proto::proto_all::Entity{id: player.id, x: pos.x, y: pos.y, angle: pos.angle});
            }
        }
    } 
}

