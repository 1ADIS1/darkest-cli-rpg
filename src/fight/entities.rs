use crate::fight::attacks::Attack;
use crate::status_effects::{EffectType, TriggerableEffect};
use crate::{Rng, HashMap, try_probability};

#[derive(Debug)]
pub struct Entity {
    image: String,
    pub name: String,

    pub health: u32,
    armor: u32,
    // dodge: u32,

    effects: Vec<Box<dyn TriggerableEffect>>, // buffs or debuffs

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
        resistances.insert(EffectType::Debuff, rng.gen_range(0..100));

        Entity {
            image: "(0 -- 0)".to_string(),
            name: "John".to_string(),

            health: rng.gen_range(0..100),
            armor: rng.gen_range(0..100),

            effects: vec![],

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
    returns true if effect was applied after trying to resist and false if not
    */
    pub fn try_apply_effect(&mut self, effect: &Box<dyn TriggerableEffect>) -> bool {        
        let final_probability = self.calculate_resist(effect.get_type(), effect.get_probability());

        // If probability succeeded -> apply effect
        if try_probability(&final_probability) {
            println!("{} got {:?}!\n", self.name, &effect.get_type());

            self.effects.push(effect);
            true
        } else {
            println!("{} resisted the {:?}.\n", self.name, &effect.get_type());

            false
        }
    }

    /**
    returns effect's probaility - resist
    */
    fn calculate_resist(&self, effect_type: &EffectType, effect_probability: &u32) -> u32 {
        let entity_resistance = self.resistances.get(effect_type);

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

    fn print_entity(&self) {
        println!("\n{}\n", self.image);

        println!("-------------------------------------------------------------");

        println!("\nStatus");
        println!("Health: {}", self.health);
        println!("Armor: {}", self.armor);
        println!("Current effects: {:?}", self.effects);

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