use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)] // WTF am I deriving?
pub enum EffectType {
    Blight,
    Stun,
    Bleed,
    Infection,
    Debuff,
    // DeathBlow,
    // Trap,
}

pub trait TriggerableEffect: Debug {
	fn trigger(&self);

	fn get_type(&self) -> &EffectType;

	fn get_probability(&self) -> &u32;
}

// Base for every effect
#[derive(Debug)]
pub struct DefaultEffect {
	effect_type: EffectType,
	probability: u32,
}

// Default implementations for every effect
impl DefaultEffect {
	fn new(effect_type: EffectType, probability: u32) -> Self {
		Self {
			effect_type,
			probability,
		}
	}
}

#[derive(Debug)]
pub struct Blight {
	default_effect: DefaultEffect,

	damage: u32,
	duration: u32,
}

impl TriggerableEffect for Blight {
	fn trigger(&self) {
		todo!()
	}

	fn get_type(&self) -> &EffectType {
		&self.default_effect.effect_type
	}

	fn get_probability(&self) -> &u32 {
		&self.default_effect.probability
	}
}

impl Blight {
	fn new(probability: u32, damage: u32, duration: u32) -> Self {
		Self {
			default_effect: DefaultEffect::new(EffectType::Blight, probability),

			damage,
			duration,
		}
	}
}