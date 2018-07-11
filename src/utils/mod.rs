pub fn iter_map_collect<F, T, U>(entities: &Vec<T>, mapper: F) -> Vec<U>
where
    F: FnMut(&T) -> U,
{
    entities.iter().map(mapper).collect()
}
