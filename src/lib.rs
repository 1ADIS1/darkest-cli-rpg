use game_state::GameState;
pub use rand::prelude::*;
pub use std::collections::HashMap;

pub mod fight;
use fight::entities::Entity;

pub mod game_state;
pub mod status_effects;

trait Death {
    fn die(&mut self);
}

pub struct Player<'a> {
    pub entity: Entity,
    game_state: &'a mut GameState,

    // inventory: HashMap<Item, u32>
}

impl<'a> Death for Player<'a> {

    fn die(&mut self) {
        self.game_state.player_dies();
    }
}

pub struct Enemy<'a> {
    pub entity: Entity,
    game_state: &'a mut GameState,
}

impl<'a> Death for Enemy<'a> {

    fn die(&mut self) {
        self.game_state.enemy_dies();
    }
}

pub fn try_probability(probability: &u32) -> bool {
    let rand_num = thread_rng().gen_range(0..100);

    *probability > rand_num
}
