use std::{cell::RefCell, collections::HashMap, hash::Hash, ops::Deref, rc::Rc};

#[derive(Debug)]
pub struct PriorityQueue<T: Eq + Hash + Clone, P: Ord + Clone, X: Clone> {
    nodes: Vec<Rc<RefCell<Node<T, P, X>>>>,
    map: HashMap<T, Rc<RefCell<Node<T, P, X>>>>,
}

#[derive(Debug, Clone)]
struct Node<T: Eq + Hash + Clone, P: Ord + Clone, X: Clone> {
    index: usize,
    data: T,
    priority: P,
    extra: X,
}

fn parent_index(index: usize) -> usize {
    (index - 1) / 2
}

fn left_child_index(index: usize) -> usize {
    2 * index + 1
}

impl<T: Eq + Hash + Clone, P: Ord + Clone, X: Clone> Clone for PriorityQueue<T, P, X> {
    fn clone(&self) -> Self {
        let nodes = self
            .nodes
            .iter()
            .map(|rc| {
                Rc::new(RefCell::new(
                    (rc as &dyn Deref<Target = RefCell<Node<T, P, X>>>)
                        .deref()
                        .borrow()
                        .clone(),
                ))
            })
            .collect::<Vec<_>>();
        let mut map = HashMap::new();
        for rc in nodes.iter() {
            map.insert(rc.borrow().data.clone(), rc.clone());
        }
        PriorityQueue { nodes, map }
    }
}

impl<T: Eq + Hash + Clone, P: Ord + Clone, X: Clone> PriorityQueue<T, P, X> {
    pub fn new() -> PriorityQueue<T, P, X> {
        PriorityQueue {
            nodes: Vec::new(),
            map: HashMap::new(),
        }
    }

    pub fn pop_by_priority(&mut self) -> Option<(T, P, X)> {
        if self.nodes.len() == 0 {
            None
        } else if self.nodes.len() == 1 {
            let root = self.nodes.pop().unwrap();
            let root_node = root.borrow();
            self.map.remove(&root_node.data);
            Some((
                root_node.data.clone(),
                root_node.priority.clone(),
                root_node.extra.clone(),
            ))
        } else {
            let result = {
                let root = self.nodes[0].clone();
                let root_node = root.borrow();
                self.map.remove(&root_node.data);
                Some((
                    root_node.data.clone(),
                    root_node.priority.clone(),
                    root_node.extra.clone(),
                ))
            };
            let last = self.nodes.pop().unwrap();
            last.borrow_mut().index = 0;
            self.nodes[0] = last;
            self.heapify(0);
            result
        }
    }

    pub fn push(&mut self, data: T, priority: P, extra: X) {
        if let Some(node) = self.map.get(&data) {
            self.heapify({
                let mut borrow = node.borrow_mut();
                borrow.priority = priority;
                borrow.index
            });
        } else {
            let node = Rc::new(RefCell::new(Node {
                index: self.nodes.len(),
                data: data.clone(),
                priority,
                extra,
            }));
            self.nodes.push(node.clone());
            self.map.insert(data, node);
            self.heapify(self.nodes.len() - 1);
        }
    }

    fn heapify(&mut self, index: usize) {
        if index != 0 {
            let parent_index = parent_index(index);
            if self.nodes[parent_index].borrow().priority > self.nodes[index].borrow().priority {
                (self.nodes[parent_index], self.nodes[index]) =
                    (self.nodes[index].clone(), self.nodes[parent_index].clone());
                self.nodes[index].borrow_mut().index = index;
                self.nodes[parent_index].borrow_mut().index = parent_index;
                self.heapify(parent_index);
            }
        }
        let left_child_index = left_child_index(index);
        if self.nodes.len() > left_child_index {
            if self.nodes[index].borrow().priority > self.nodes[left_child_index].borrow().priority
            {
                (self.nodes[index], self.nodes[left_child_index]) = (
                    self.nodes[left_child_index].clone(),
                    self.nodes[index].clone(),
                );
                self.nodes[index].borrow_mut().index = index;
                self.nodes[left_child_index].borrow_mut().index = left_child_index;
                self.heapify(left_child_index);
            }
            if self.nodes.len() > left_child_index + 1
                && self.nodes[index].borrow().priority
                    > self.nodes[left_child_index + 1].borrow().priority
            {
                (self.nodes[index], self.nodes[left_child_index + 1]) = (
                    self.nodes[left_child_index + 1].clone(),
                    self.nodes[index].clone(),
                );
                self.nodes[index].borrow_mut().index = index;
                self.nodes[left_child_index + 1].borrow_mut().index = left_child_index + 1;
                self.heapify(left_child_index + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec<T: Eq + Hash + Clone, P: Ord + Clone, X: Clone>(
        pq: &PriorityQueue<T, P, X>,
    ) -> Vec<T> {
        let mut result = Vec::new();
        let mut pq = pq.clone();

        while let Some((data, _, _)) = pq.pop_by_priority() {
            result.push(data);
        }

        result
    }

    #[test]
    fn without_extra() {
        let mut pq = PriorityQueue::<i32, usize, ()>::new();
        assert_eq!(to_vec(&pq), vec![]);

        pq.push(42, 42, ());
        assert_eq!(to_vec(&pq), vec![42]);

        pq.push(1, 1, ());
        assert_eq!(to_vec(&pq), vec![1, 42]);

        pq.push(2, 2, ());
        assert_eq!(to_vec(&pq), vec![1, 2, 42]);

        pq.push(100, 100, ());
        assert_eq!(to_vec(&pq), vec![1, 2, 42, 100]);

        pq.push(42, 0, ());
        assert_eq!(to_vec(&pq), vec![42, 1, 2, 100]);

        _ = pq.pop_by_priority();
        assert_eq!(to_vec(&pq), vec![1, 2, 100]);

        pq.push(42, 42, ());
        assert_eq!(to_vec(&pq), vec![1, 2, 42, 100]);
    }
}
