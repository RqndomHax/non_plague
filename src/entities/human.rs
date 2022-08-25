use crate::traits::{breedable::Breedable, Trait};

#[derive(PartialEq, Eq)]
pub enum HumanGender {
    Female,
    Male,
}

#[derive(PartialEq, Eq)]
pub struct Human {
    gender: HumanGender,
    traits: Vec<Trait>,
}

impl Human {
    pub fn spawn(gender: HumanGender, entities: &mut Vec<Human>) {
        entities.push(Self::generate(gender));
    }

    pub fn generate(gender: HumanGender) -> Human {
        Human {
            gender,
            traits: Vec::new(),
        }
    }
}

impl Breedable<Human> for Human {
    fn breed(entity_1: Human, entity_2: Human) -> Option<Human> {
        if !entity_1.traits.contains(&Trait::Breedable)
            || !entity_2.traits.contains(&Trait::Breedable)
        {
            return None;
        }

        // Do not breed the entity if it's of the same type.
        if entity_1 == entity_2 {
            return None;
        }

        // Generate true or false (0.5 probability).
        match rand::random::<bool>() {
            true => Some(Self::generate(HumanGender::Male)),
            false => Some(Self::generate(HumanGender::Female)),
        }
    }
}
