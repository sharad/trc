use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

struct Node<K, V> {
    key: K,
    value: V,
    prev: usize,
    next: usize,
}

pub struct Lru<K: Eq + Hash, V: Clone> {
    capacity: usize,
    store: HashMap<K, usize>,
    nodes: Vec<Node<K, V>>,
    head: usize,
    tail: usize,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, value: V) -> Self {
        Node {
            key,
            value,
            next: 0,
            prev: 0,
        }
    }
}

impl<K: Clone + Ord + Eq + std::hash::Hash, V: Clone + Debug> Lru<K, V> {
    pub fn new(capacity: usize) -> Self {
        Lru {
            capacity,
            store: HashMap::new(),
            nodes: Vec::with_capacity(capacity),
            head: 0,
            tail: 0,
        }
    }

    pub fn debug(&self, tag: &str) {
        eprintln!(
            "{}: head: {} / tail: {} / len: {}",
            tag,
            self.head,
            self.tail,
            self.nodes.len()
        );
        eprintln!();
        for (i, n) in self.nodes.iter().enumerate() {
            eprintln!(
                "{}:   {}: {:?}, next: {}/ prev: {}",
                tag,
                i + 1,
                n.value,
                n.next,
                n.prev
            );
        }
        eprintln!();
    }

    pub fn put(&mut self, k: K, v: V) {
        let pos = if let Some(&indx) = self.store.get(&k) {
            self.nodes[indx - 1].value = v;
            indx
        } else {
            let mut node = Node::new(k, v.clone());
            if self.nodes.len() >= self.capacity {
                self.store.remove(&(self.nodes[self.tail - 1].key));
                node.prev = self.nodes[self.tail - 1].prev;
                self.nodes[self.tail - 1] = node;
                self.store
                    .insert(self.nodes[self.tail - 1].key.clone(), self.tail);
                self.tail
            } else {
                self.nodes.push(node);
                let indx = self.nodes.len();
                self.store.insert(self.nodes[indx - 1].key.clone(), indx);
                indx
            }
        };
        self.move_ahead(pos);
    }

    pub fn get(&mut self, k: K) -> Option<V> {
        if let Some(&indx) = self.store.get(&k) {
            self.move_ahead(indx);
            Some(self.nodes[indx - 1].value.clone())
        } else {
            None
        }
    }

    fn move_ahead(&mut self, indx: usize) {
        if self.head == indx {
            return;
        }

        let prev = self.nodes[indx - 1].prev;
        let next = self.nodes[indx - 1].next;

        if prev > 0 {
            self.nodes[prev - 1].next = next;
        }

        if next > 0 {
            self.nodes[next - 1].prev = prev;
        }

        if self.tail == indx {
            self.tail = prev;
        }

        self.nodes[indx - 1].prev = 0;
        self.nodes[indx - 1].next = self.head;

        if self.head > 0 {
            self.nodes[self.head - 1].prev = indx;
        }

        self.head = indx;

        if self.tail == 0 {
            self.tail = indx;
        }
    }
}

#[cfg(test)]
mod tests {
    use core::assert;

    use super::*;

    #[test]
    fn test_lru1() {
        let mut lru = Lru::<&str, String>::new(2);
        lru.put("test1", "Hello1".to_string());
        // assert_eq!("Hello1", lru.get("test1").unwrap());
        // lru.debug("1");
        lru.put("test2", "Hello2".to_string());
        lru.put("test3", "Hello3".to_string());
        lru.put("test4", "Hello4".to_string());
        assert_eq!("Hello4", lru.get("test4").unwrap());
        assert!(lru.get("test1").is_none());
        assert!(lru.get("test2").is_none());
        assert_eq!("Hello3", lru.get("test3").unwrap());
    }

    #[test]
    fn test_lru2() {
        let mut lru = Lru::<&str, String>::new(2);
        lru.put("test1", "Hello1".to_string());
        lru.put("test2", "Hello2".to_string());
        assert_eq!("Hello1", lru.get("test1").unwrap());
        lru.put("test3", "Hello3".to_string()); // test2 removed
        assert_eq!("Hello1", lru.get("test1").unwrap());
        lru.put("test4", "Hello4".to_string()); // test3 removed
        assert_eq!("Hello4", lru.get("test4").unwrap()); // test1 and test4 will
        // be left
        assert!(lru.get("test2").is_none());
        assert!(lru.get("test3").is_none());
        assert_eq!("Hello1", lru.get("test1").unwrap());
        assert_eq!("Hello4", lru.get("test4").unwrap());
    }

    #[test]
    fn test_update_existing() {
        let mut lru = Lru::<&str, String>::new(2);

        lru.put("a", "1".to_string());
        lru.put("b", "2".to_string());

        lru.put("a", "10".to_string());

        assert_eq!("10", lru.get("a").unwrap());

        lru.put("c", "3".to_string());

        assert!(lru.get("b").is_none());

        assert_eq!("10", lru.get("a").unwrap());
    }

    #[test]
    fn test_get_refreshes_lru() {
        let mut lru = Lru::<&str, String>::new(2);

        lru.put("a", "A".to_string());
        lru.put("b", "B".to_string());

        assert_eq!("A", lru.get("a").unwrap());

        lru.put("c", "C".to_string());

        assert!(lru.get("b").is_none());

        assert_eq!("A", lru.get("a").unwrap());
        assert_eq!("C", lru.get("c").unwrap());
    }

    #[test]
    fn test_no_duplicate_links() {
        let mut lru = Lru::<&str, String>::new(3);

        lru.put("a", "a".into());
        lru.put("b", "b".into());
        lru.put("c", "c".into());

        lru.get("b");
        lru.get("b");
        lru.get("b");

        lru.debug("after");
    }

    #[test]
    fn test_all_nodes_reachable_forward() {
        let mut lru = Lru::<&str, String>::new(3);

        lru.put("a", "A".into());
        lru.put("b", "B".into());
        lru.put("c", "C".into());

        lru.debug("forward1");

        lru.get("b");

        lru.debug("forward2");

        let mut count = 0;

        let mut cur = lru.head;
        eprintln!("cur: {}", cur);
        while cur > 0 {
            count += 1;
            cur = lru.nodes[cur - 1].next;
            eprintln!("next: {}", cur);

            assert!(count < 10);
        }

        assert_eq!(count, lru.store.len());

        // assert_eq!(8, 4);
    }

    #[test]
    fn test_all_nodes_reachable_backward() {
        let mut lru = Lru::<&str, String>::new(3);

        lru.put("a", "A".into());
        lru.put("b", "B".into());
        lru.put("c", "C".into());

        lru.debug("backward1");

        lru.get("b");

        lru.debug("backward2");

        let mut count = 0;
        let mut cur = lru.tail;

        eprintln!("cur: {}", cur);

        while cur > 0 {
            count += 1;

            cur = lru.nodes[cur - 1].prev;
            eprintln!("prev: {}", cur);

            assert!(count < 10);
        }

        assert_eq!(count, lru.store.len());
    }

    #[test]
    fn test_bidirectional_consistency() {
        let mut lru = Lru::<&str, String>::new(3);

        lru.put("a", "A".into());
        lru.put("b", "B".into());
        lru.put("c", "C".into());

        lru.get("b");

        for i in 0..lru.nodes.len() {
            let n = &lru.nodes[i];

            if n.next > 0 {
                assert_eq!(lru.nodes[n.next - 1].prev, i + 1);
            }

            if n.prev > 0 {
                assert_eq!(lru.nodes[n.prev - 1].next, i + 1);
            }
        }
    }

    #[test]
    fn test_many_refreshes() {
        let mut lru = Lru::<usize, usize>::new(5);

        for i in 0..5 {
            lru.put(i, i);
        }

        for _ in 0..100 {
            lru.get(2);
            lru.get(4);
            lru.get(1);
        }

        assert!(lru.get(0).is_some());

        lru.put(10, 10);

        assert!(lru.get(3).is_none());
    }

    #[test]
    fn test_move_tail_to_head() {
        let mut lru = Lru::<usize, usize>::new(4);

        for i in 1..=4 {
            lru.put(i, i);
        }

        lru.get(1);

        assert_eq!(Some(1), lru.get(1));

        lru.put(5, 5);

        assert!(lru.get(2).is_none());
    }
}
