


use std::collections::HashMap;

type Link<K, V> = Option<Arc<RefCell<Node<K, V>>>>;


struct Node<K, V> {
    key: K,
    value: V,
    prev: Link<K, V>,
    next: Link<K, V>,
}

pub struct Lru<K: Eq+std::hash::Hash, V: Clone> {
    capacity: usize,
    size: usize,
    store: Mutex<HashMap<K, V>>,
    head: Link<K, V>,
    tail: Link<K, V>,
}

impl<K, V> Node<K, V> {
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

        n = Some(Arc::new(RefCell.new(Node.new(k, v)));)

        match Self.head {
            None => {
                Self.head  = Self.tail = n;
                n.next = n.prev = n

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
