


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
    fn new(key: &'a K, value: &'a V) -> Self {
        Node{
            key,
            value,
            None,
            None
        }
    }
}

impl<'a, K: Ord, V> lru<'a, K, V> {

    pub fn new(capacity: usize) -> Self {
        Lru{
            capacity,
            size: 0,
            None,
            None
        }
    }

    fn headInsert(&mut Self, k: &'a K, v: &'a V) {
        match Self.head {
            None => {
                Self.head  = Self.tail = Some(Box(Node.new(k, v)));

            }
            Some(Box( Node {} )) => {
            }
        }
    }
    fn tailRelease(&mut Self, ) {
    }


    pub fn get(&mut Self, k: &'a K) -> &'a V{
    }
    pub fn put(&mut Self, k: &'a K, v: &'a V) {
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
