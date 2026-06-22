


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
    nodes: Vec<Node<K, V>>
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

impl<K: Ord, V> lru<K, V> {

    pub fn new(capacity: usize) -> Self {
        Lru{ capacity }
    }

    fn headInsert(&mut self, k: &'a K, v: &'a V) {

        self.nodes.push(Node.new(k, v))

        match self.head {
            0 => {
                self.head  = self.tail = 1;
                n.next = n.prev = 1;
            }
            indx => {
                n.next = self.head;
                n.prev = self.tail;
                self.head = self.nodes.size() + 1;
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
