use crate::{*, status_effects::TriggerableEffect};

#[derive(Debug)]
pub struct Attack {
    pub name: String,
    pub effects: Vec<Box<dyn TriggerableEffect>>,
    pub damage: u32,
    // accuracy: u32,
    // crit: u32,
}

impl Attack {
    pub fn use_ability(&self, entity_to_attack: &mut Entity)
    {
        entity_to_attack.receive_damage(self.damage);

        for effect in &self.effects {
            entity_to_attack.try_apply_effect(effect);
        }
    }
}