pub fn iter_map_collect<F, T, U>(entities: &Vec<T>, mapper: F) -> Vec<U>
where
    F: FnMut(&T) -> U,
{
    entities.iter().map(mapper).collect()
}

pub fn clear_console() {
    print!("{}[2J", 27 as char);
}

pub fn replace_item<T>(index: &usize, entities: &mut Vec<T>, new_item: T) {
    let index = index.clone();
    entities.remove(index);
    entities.insert(index, new_item);
}
