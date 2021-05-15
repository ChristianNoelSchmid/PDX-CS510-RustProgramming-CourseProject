use std::time::Instant;

use simple_serializer::Serialize;

use crate::{
    astar::visible_actors,
    state::{
        stats::{Attributes, Stats},
        traits::{Combater, Identified, Translator, AI},
        transforms::vec2::Vec2,
    },
};

use super::transforms::world_stage::WorldStage;

///
/// Represents a particular monster type
/// in the game. The reason `name` is static
/// is to communicate that there are templates,
/// rather than the Monster objects themselves.
///
#[derive(Clone)]
pub struct Monster {
    pub stats: Stats,
    pub attrs: Attributes,
    pub sight_range: u32,

    pub id: u32,
    pub name: &'static str,
    pub spawn_chance: u32,
}

#[derive(Clone)]
pub struct MonsterInstance {
    pub template: &'static Monster,
    pub instance_id: u32,
    pub path: Vec<Vec2>,

    pub combat_target: Option<u32>,
    pub last_sighting: Instant,
}

impl MonsterInstance {
    pub fn new(template: &'static Monster, instance_id: u32) -> Self {
        Self {
            template,
            instance_id,
            path: Vec::new(),
            combat_target: None,
            last_sighting: Instant::now(),
        }
    }
}

impl Identified for MonsterInstance {
    fn id(&self) -> u32 {
        self.instance_id
    }
}

impl Translator for MonsterInstance {
    fn target(&self) -> Option<&Vec2> {
        self.path.first()
    }
    fn set_path(&mut self, path: Vec<Vec2>) {
        self.path = path;
    }
    fn next_step(&mut self) -> Option<Vec2> {
        self.path.pop()
    }
}

impl Combater for MonsterInstance {
    fn combat_target(&self) -> Option<u32> {
        self.combat_target
    }
    fn start_combat_with(&mut self, id: u32) {
        self.combat_target = Some(id)
    }
    fn stop_combat(&mut self) {
        self.combat_target = None;
    }
    fn sight_range(&self) -> u32 {
        self.template.sight_range
    }
    fn last_sighting(&self) -> Instant {
        self.last_sighting
    }
    fn reset_last_sighting(&mut self) {
        self.last_sighting = Instant::now();
    }
}

impl AI for MonsterInstance {}

impl Serialize for MonsterInstance {
    type SerializeTo = String;
    fn serialize(&self) -> Self::SerializeTo {
        // Serializing `MonsterInstance` only requires
        // the template id from `Monster`. The client
        // will have the rest of the information to
        // generate the `MonsterInstance` from it's
        // associated template id.
        format!("{}::{}", self.template.id, self.instance_id,)
    }
}
