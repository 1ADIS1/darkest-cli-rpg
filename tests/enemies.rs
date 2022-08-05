use darkest_cli_rpg::{fight::entities::Entity, EffectType, try_probability};

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

	for i in 0..test_amount {
		results.push(try_probability(&probability_to_test));
	}

	let falses = results.into_iter()
		.filter(|x| *x == false)
		.collect::<Vec<bool>>()
		.len();

	println!("Number of false is {}", falses);
	println!("Number of true is {}", test_amount - falses);
}