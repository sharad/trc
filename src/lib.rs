


use std::collections::HashMap;


struct Node<'a, K, V> {
    key: &'a K,
    value: &'a V,
    prev: Option<Box<Node<'a, K, V>>>,
    next: Option<Box<Node<'a, K, V>>>,
}

pub struct Lru<'a, K, V> {
    capacity: usize,
    size: usize,
    store: HashMap<K, V>,
    head: Option<Box<Node<'a, K, V>>>,
    tail: Option<Box<Node<'a, K, V>>>,
}

impl<'a, K: Ord, V> Node<'a, K, V> {

    fn new() -> Self {
        Node()
    }

    fn headInsert(k: &'a K, v: &'a V) {
    }
    fn tailRelease(k: &'a K) {
    }
}

impl<'a, K: Ord, V> lru<'a, K, V> {
    pub fn get(k: &'a K) -> &'a V{
    }
    pub fn put(k: &'a K, v: &'a V) {
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
