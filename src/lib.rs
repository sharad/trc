


use std::collections::HashMap;

type Link<K, V> = Option<Rc<RefCell<Node<K, V>>>>;


struct Node<'a, K, V> {
    key: K,
    value: V,
    prev: Link<K, V>,
    next: Link<K, V>,
}

pub struct Lru<'a, K: Eq+std::hash::Hash, V: Clone> {
    capacity: usize,
    size: usize,
    store: Mutex<HashMap<K, V>>,
    head: Link<K, V>,
    tail: Link<K, V>,
}

impl<K: Ord, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
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


    pub fn get(&mut self, k: &'a K) -> &'a V{
    }
    pub fn put(&mut self, k: &'a K, v: &'a V) {
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
