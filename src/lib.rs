


use std::collections::HashMap;

type Link<K, V> = Option<Arc<RefCell<Node<K, V>>>>;


struct Node<K, V> {
    key: K,
    value: V,
    prev: Option<usize>,
    next: Option<usize>,
}

pub struct Lru<K: Eq+std::hash::Hash, V: Clone> {
    // Mutex
    capacity: usize,
    store: HashMap<K, V>,
    head: usize,
    tail: usize,
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
            None,
            None
        }
    }

    fn headInsert(&mut Self, k: &'a K, v: &'a V) {

        n = Some(Arc::new(RefCell.new(Node.new(k, v)));)

        match Self.head {
            None => {
                self.head  = Self.tail = n;
                n.next = n.prev = n;
            }
            Some(Arc( Ref(Node { head, tail }) )) => {
                n.next = head;
                n.prev = tail;
                head = n;
                // tail = 

            }
        }
    }
    fn tailRelease(&mut self) {
        match self.tail {
            None => {}
            Some(Arc( Ref(Node { tail }) )) => {
                tail = tail.prev;
            }
        }
    }
    fn findLink(key: &K) {
        
    }
    fn moveAhead(link: Link<K, V>){
        
    }

    pub fn get(&mut self, k: K) -> Option<V>{
        store.get(k).cloned()
    }
    pub fn put(&mut self, k: K, v: V) {
        store.
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
