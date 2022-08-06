use crate::{*, status_effects::StatusEffects};

#[derive(Debug, Clone)]
pub struct Attack {
    pub name: String,

    //Introduce hash map, where type of effect will be key 
    pub effects: StatusEffects,
    pub damage: u32,
    // accuracy: u32,
    // crit: u32,
}

impl Attack {

    pub fn use_ability(&self, entity_to_attack: &mut Entity)
    {
        entity_to_attack.receive_damage(self.damage);

        entity_to_attack.try_apply_effects(&self.effects);
    }
}