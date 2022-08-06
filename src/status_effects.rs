use std::fmt::Debug;

use crate::*;

/**
contains each type of the effects
*/
#[derive(Debug, Clone)]
pub struct StatusEffects {
	// pub triggerable_effects: Vec<Box<dyn TriggerableEffect>>,
	pub damageable_effects: Vec<DamageableEffect>,
    // other status effects
}

impl StatusEffects {

	// Add initialization for other effects
	pub fn new() -> Self {
		Self {
			damageable_effects: vec![],
		}
	}

	pub fn add_damageable_effect(&mut self, effect: DamageableEffect) {
		self.damageable_effects.push(effect);
	}

	// pub fn add_debuff_effects

	// pub fn add_passive_effects

	pub fn trigger_effects(&self, entity: &mut Entity) {

		
	}
}

// TODO: Can I declare the EffectType as the Effect Struct? Should I?
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

pub trait TriggerableEffect: Debug + Clone {
	fn trigger(&mut self, entity: &mut Entity);
}

#[derive(Debug, Clone)]
pub struct DamageableEffect {
	pub default_effect: DefaultEffect,
	pub damage: u32,
	pub duration: u32,
}

impl DamageableEffect {
	pub fn new(effect_type: EffectType, probability: u32, damage: u32, duration: u32) -> Self {
		Self {
			default_effect: DefaultEffect::new(effect_type, probability),
			damage,
			duration,
		}
	}	
}

impl TriggerableEffect for DamageableEffect {
	fn trigger(&mut self, entity: &mut Entity) {
	    entity.receive_damage(self.damage);
	}
}

// Base for every effect
#[derive(Debug, Clone)]
pub struct DefaultEffect {
	pub effect_type: EffectType,
	pub probability: u32,
}

// Default implementations for every effect
impl DefaultEffect {
	pub fn new(effect_type: EffectType, probability: u32) -> Self {
		Self {
			effect_type,
			probability,
		}
	}
}
