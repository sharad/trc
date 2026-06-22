


use std::collections::HashMap;


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

    fn put(&mut self, k: K, v: V) {

        let indx = self.store.get(k).unwrap();
        if indx {
            self.nodes[ indx ] = Node.new(k, v)
        } else {
            self.nodes.push(Node.new(k, v));
            indx = self.nodes.size();
            store.insert(k, indx)
        }
        moveAhead(indx);
    }


    pub fn get(&mut self, k: K) -> Option<V>{
        let indx = self.store.get(k).unwrap();
        if indx {
            self.nodes[ indx ] = Node.new(k, v)
        } else {
            self.nodes.push(Node.new(k, v));
            indx = self.nodes.size();
            store.insert(k, indx)
        }
    }



    fn drop(&mut self) {
        match self.tail {
            0 => {}
            indx => {
                tail = self.nodes[indx].prev;
            }
        }
    }
    fn moveAhead(self, indx: usize) -> usize {
        // indx = self.store.get(k).unwrap();
        nodes[indx].next = self.head;
        nodes[indx].prev = None;
        self.head = indx;
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
