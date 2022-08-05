use crate::{EffectType, Attack, Rng, HashMap, try_probability};

#[derive(Debug)]
pub struct Entity {
    image: String,
    name: String,

    health: u32,
    armor: u32,
    effects: HashMap<EffectType, u32>, // buffs or debuffs

    // TODO: Make sure that every EffectType is initialized
    pub resistances: HashMap<EffectType, u32>,

    attacks: Vec<Attack>,
}

impl Entity {

    /**
    Get random entity from one of the predefined creatures with random stats
    TODO: random abilities and effects
    */
    pub fn random() -> Entity {
        let mut rng = rand::thread_rng();
        let mut resistances = HashMap::new();

        resistances.insert(EffectType::Blight, rng.gen_range(0..100));
        resistances.insert(EffectType::Stun, rng.gen_range(0..100));
        resistances.insert(EffectType::Bleed, rng.gen_range(0..100));
        resistances.insert(EffectType::Infection, rng.gen_range(0..100));

        Entity {
            image: "(0 -- 0)".to_string(),
            name: "John".to_string(),

            health: rng.gen_range(0..100),
            armor: rng.gen_range(0..100),

            effects: HashMap::new(),

            resistances,

            attacks: Vec::new(),
        }
    }

    fn receive_damage(&mut self, amount: u32) {
        self.health -= amount;

        if self.health == 0 {
            self.die();
        }
    }

    fn die(&self) {
        todo!()
    }

    pub fn encounter(&self) {
        todo!()
    }

    /**
    returns true if effect was applied after trying to resist and false if not
    */
    pub fn try_apply_effect(&mut self, effect: (EffectType, u32)) -> bool {
        
        let final_probability = self.calculate_resist(&effect.0,
            &effect.1);

        // If probability succeeded -> apply effect
        if try_probability(&final_probability) {
            self.effects.insert(effect.0, effect.1);
            true
        } else {
            false
        }
    }

    /**
    returns effect's probaility - resist
    */
    fn calculate_resist(&self, effect: &EffectType, effect_probability: &u32) -> u32 {
        let entity_resistance = self.resistances.get(effect);

        let entity_resistance: u32 = match entity_resistance {
            Some(resistance_probability) => *resistance_probability,
            None => {
                println!("Resistance is not initialized!");
                0
            },
        };

        if entity_resistance > *effect_probability {
            0
        } else {
            *effect_probability - entity_resistance
        }
    }
}