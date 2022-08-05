#![allow(dead_code, unused_variables)]

pub use rand::prelude::*;
pub use std::collections::{HashMap};

pub mod fight;
use fight::entities::Entity;

#[derive(Debug)]
struct Attack {
    name: String,
    effects: HashMap<EffectType, u32>,
    damage: u32,
}

trait Applicable {
    fn apply_to(&self, entity: Entity);
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)] // WTF am I deriving?
pub enum EffectType {
    Blight,
    Stun,
    Bleed,
    Infection,
}

impl Applicable for EffectType {
    fn apply_to(&self, entity: Entity) {
        // entity.try_apply_effect(self);
        todo!()
    }
}

impl Attack {
    fn use_ability(&self) {
        todo!()
    }
}

fn run() {
    let new_enemy = Entity::random();

    new_enemy.encounter();

    todo!()
}

fn welcoming_messages() {

}

pub fn try_probability(probability: &u32) -> bool {
    let rand_num = thread_rng().gen_range(0..100);

    *probability > rand_num
}
