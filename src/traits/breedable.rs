pub trait Breedable<T> {
    fn breed(entitiy_1: T, entity_2: T) -> Option<T>;
}
