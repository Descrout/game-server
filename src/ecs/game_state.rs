use crate::proto::proto_all;
#[derive(Debug, Clone)]
pub struct GameState{
    pub id: u32,
    pub state: proto_all::State,
}

impl GameState{
    pub fn new(id: u32) -> Self{
        Self{
            id,
            state: proto_all::State{last_seq: 0, entities: Vec::new()},
        }
    }

    pub fn clear(&mut self) {
        self.state.entities.clear();
    }

    pub fn add_entity(&mut self, entity: proto_all::Entity) {
        self.state.entities.push(entity);
    }
}