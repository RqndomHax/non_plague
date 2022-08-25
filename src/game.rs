use crate::entities::{human::*, virus::*};

pub struct Game {
    pub humans: Vec<Human>,
    pub virus: Virus,
}

impl Game {
    pub fn test_entity(&mut self) {
        Human::spawn(HumanGender::Female, &mut self.humans);
        println!("humans size = {}", self.humans.len());
    }

    pub fn init() -> Self {
        Game {
            humans: Vec::new(),
            virus: Virus::Hepatitis,
        }
    }
}
