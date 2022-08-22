use crate::traits::breedable::*;

#[derive(PartialEq, Eq)]
pub enum Human {
    Female,
    Male,
}

impl Human {
    pub fn spawn(entity: Human, entities: &mut Vec<Human>) {
        entities.push(entity);
    }
}

impl Breedable<Human> for Human {
    fn breed(entity_1: Human, entity_2: Human) -> Option<Human> {
        // Do not breed the entity if it's of the same type.
        if entity_1 == entity_2 {
            return None;
        }

        // Generate true or false (0.5 probability).
        match rand::random::<bool>() {
            true => Some(Human::Male),
            false => Some(Human::Female),
        }
    }
}
