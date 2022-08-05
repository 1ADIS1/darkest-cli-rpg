use darkest_cli_rpg::fight::{attacks::Attack, entities::Entity};
use crate::status_effects::EffectType;
use darkest_cli_rpg::*;

fn debug_entity() -> Entity {
	let new_entity = Entity::random();
	println!("New entity is {:?}", new_entity);

	return new_entity;
}

#[test]
#[ignore]
fn test_player_receive_death_blow() {
	//let player = Player { health: 100};

	//assert_eq!(darkest_cli_rpg::Player::receive_damge(100), 0);

	todo!()
}

#[test]
fn test_negative_damage() {
	let health: u32 = 10;

	assert_eq!(health - 10, 0);
}

#[test]
fn test_random_entity() {
	let new_entity = Entity::random();

	println!("{:?}", new_entity);
}

#[test]
fn try_apply_effect_or_resist() {
	let mut new_entity = Entity::random();

	new_entity.resistances.insert(EffectType::Blight, 10);

	let blight_effect = (EffectType::Blight, 90);

	let resist_result = new_entity.try_apply_effect(blight_effect);

	println!("Blight resist result: {resist_result}");
}

#[test]
fn test_probabilities() {
	let test_amount = 1000;
	let mut results = Vec::new();

	let probability_to_test: u32 = 100;

	for _ in 0..test_amount {
		results.push(try_probability(&probability_to_test));
	}

	let falses = results.into_iter()
		.filter(|x| *x == false)
		.collect::<Vec<bool>>()
		.len();

	println!("Number of false is {}", falses);
	println!("Number of true is {}", test_amount - falses);
}

#[test]
fn try_ability_on_enemy() {
	let mut new_entity = debug_entity();

	new_entity.health = 10;

	let new_bleed_effect = Effect {
		effect_type: EffectType::Bleed,
		duration: 3,
		damage: 4,
	};

	let mut ability_effects = vec![new_bleed_effect];

	let new_ability = Attack {
		name: "Feral claws".to_string(),
		effects: ability_effects,
		damage: 5,
	};

	println!("\nTrying to attack entity with {:?}", new_ability);

	new_ability.use_ability(&mut new_entity);

	assert_eq!(new_entity.health, 5);
}

#[test]
fn try_debuff() {
	let new_debuff_effect = DebuffEffect {
		effect_type: EffectType::Debuff,
		duration: 2,
		debuffs: {
			let new_debuffs = HashMap::new();

			new_debuffs.insert(Status::Damage, 10);
			new_debuffs.insert(EffectType::Bleed, 25);

			new_debuffs
		},
	};
}
