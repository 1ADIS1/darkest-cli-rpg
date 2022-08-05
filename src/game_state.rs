use super::*;

pub struct GameState {
    player: Option<Entity>,
    current_enemy: Option<Entity>,
    
    phase: Phase
}

pub enum Phase {
    Idle,
    Attack,
    Reward,
}

impl Phase {
    fn next(&self) {
        todo!()
    }
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            player: None,
            current_enemy: None,

            phase: Phase::Idle,
        };

        todo!()
    }

    pub fn run() {
        let new_game_state = GameState::new();

        todo!()
    }

    /**
    if enemy exists -> kill him and go to the next phase
    */
    pub fn enemy_dies(&mut self) {

        match &self.current_enemy {
            Some(enemy) => {
                println!("{} dies!", enemy.name);

                self.current_enemy = None;

                self.phase.next();
            },
            None => println!("Enemy is already dead."),
        };
    }

    /**
    if player exists -> kill him and go to the next phase
    */
    pub fn player_dies(&mut self) {

        match &self.player {
            Some(player) => {
                println!("{} dies!", player.name);

                self.player = None;

                self.phase.next();
            },
            None => println!("Player is already dead."),
        };

        todo!()
    }
}

fn welcoming_messages() {

}