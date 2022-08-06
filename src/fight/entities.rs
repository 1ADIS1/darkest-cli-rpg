use crate::fight::attacks::Attack;
use crate::status_effects::{StatusEffects, EffectType};
use crate::{Rng, HashMap, try_probability};

#[derive(Debug, Clone)]
pub struct Entity {
    pub image: String,
    pub name: String,

    pub health: u32,
    pub armor: u32,
    // dodge: u32,

    pub status_effects: StatusEffects,

    pub resistances: HashMap<EffectType, u32>,

    pub attacks: Vec<Attack>,
}

impl Entity {

    /**
    Get random entity from one of the predefined creatures with random stats
    TODO: random abilities and damageable_effects
    */
    pub fn random() -> Entity {
        let mut rng = rand::thread_rng();
        let mut resistances = HashMap::new();

        resistances.insert(EffectType::Blight, rng.gen_range(0..100));
        resistances.insert(EffectType::Stun, rng.gen_range(0..100));
        resistances.insert(EffectType::Bleed, rng.gen_range(0..100));
        resistances.insert(EffectType::Infection, rng.gen_range(0..100));
        resistances.insert(EffectType::Debuff, rng.gen_range(0..100));

        Entity {
            image: "(0 -- 0)".to_string(),
            name: "John".to_string(),

            health: rng.gen_range(0..100),
            armor: rng.gen_range(0..100),

            status_effects: StatusEffects::new(),

            resistances,

            attacks: Vec::new(),
        }
    }

    pub fn receive_damage(&mut self, amount: u32) {
        if self.health > amount {
            println!("{} receives {} damage!\n", self.name, amount);

            self.health -= amount;
        } else {
            self.health = 0;

            println!("{} dies!\n", self.name);

            // self.die();
        }
    }

    pub fn encounter(&self) {
        println!("\nYou've encountered...");
        println!("A {}!\n", self.name);

        self.print_entity();
    }

    /**
    In the each round entity will triggers existing damage effects on it
    */
    pub fn trigger_damage_effects(&mut self) {
        let damageable_effects = self.status_effects.damageable_effects.clone();

        for damageable_effect in damageable_effects {
            self.receive_damage(damageable_effect.damage)
        }
    }

    /**
    iterates through all of the status effects and tries to resist/apply them
    */
    // TODO: refactor the code to not duplicate loops for each type of the effect
    pub fn try_apply_effects(&mut self, effects: &StatusEffects) {

        // for every damageable, debuff and passive effects do:
        let damageable_effects = &effects.damageable_effects;
        // let debuff_effects
        // let passive_effects

        for effect in damageable_effects {
            let final_probability = self.calculate_resist(
                &effect.default_effect.effect_type, 
                effect.default_effect.probability
            );

            // If probability succeeded -> apply effect
            if try_probability(final_probability) {
                println!("{} got {:?}!\n", self.name, effect.default_effect.effect_type);

                let effect_to_apply = effect.clone();

                self.status_effects.damageable_effects.push(effect_to_apply);
            } else {
                println!("{} resisted the {:?}.\n", self.name, effect.default_effect.effect_type);
            }
        }
    }

    /**
    returns effect's probaility - resist
    */
    fn calculate_resist(&self, effect_type: &EffectType, effect_probability: u32) -> u32 {
        let entity_resistance = self.resistances.get(effect_type);

        let entity_resistance: u32 = match entity_resistance {
            Some(resistance_probability) => *resistance_probability,
            None => {
                println!("Resistance is not initialized!");
                0
            },
        };

        if entity_resistance > effect_probability {
            0
        } else {
            effect_probability - entity_resistance
        }
    }

    fn print_entity(&self) {
        println!("\n{}\n", self.image);

        println!("-------------------------------------------------------------");

        println!("\nStatus");
        println!("Health: {}", self.health);
        println!("Armor: {}", self.armor);
        // TODO: Print every effect in one line to not overcomplicate
        println!("Current effects: {:?}", self.status_effects.damageable_effects);

        println!("\nResistances");
        println!("Blight: {}%", self.resistances.get(&EffectType::Blight).unwrap());    
        println!("Stun: {}%", self.resistances.get(&EffectType::Stun).unwrap());
        println!("Bleed: {}%", self.resistances.get(&EffectType::Bleed).unwrap());
        println!("Infection: {}%", self.resistances.get(&EffectType::Infection).unwrap());
        println!("Debuff: {}%", self.resistances.get(&EffectType::Debuff).unwrap());

        println!("\nAbilities");
        println!("");

        println!("-------------------------------------------------------------");
    }
}
