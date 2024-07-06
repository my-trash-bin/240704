use std::{
    cell::RefCell,
    collections::HashMap,
    hash::Hash,
    rc::{Rc, Weak},
};

#[derive(Debug)]
pub struct PriorityQueue<T: Eq + Hash + Clone, P: Ord + Clone> {
    internal: Rc<RefCell<PriorityQueueInternal<T, P>>>,
}

#[derive(Debug)]
struct PriorityQueueInternal<T: Eq + Hash + Clone, P: Ord + Clone> {
    nodes: Vec<PriorityQueueNode<T, P>>,
    map: HashMap<T, PriorityQueueNode<T, P>>,
}

#[derive(Debug)]
struct PriorityQueueNodeInternal<T: Eq + Hash + Clone, P: Ord + Clone> {
    of: Weak<RefCell<PriorityQueueInternal<T, P>>>,
    parent: Option<Weak<RefCell<PriorityQueueNodeInternal<T, P>>>>,
    left: Option<Rc<RefCell<PriorityQueueNodeInternal<T, P>>>>,
    right: Option<Rc<RefCell<PriorityQueueNodeInternal<T, P>>>>,
    index: usize,
    priority: P,
    data: T,
}

#[derive(Debug)]
pub struct PriorityQueueNode<T: Eq + Hash + Clone, P: Ord + Clone> {
    internal: Rc<RefCell<PriorityQueueNodeInternal<T, P>>>,
}

impl<T: Eq + Hash + Clone, P: Ord + Clone> Clone for PriorityQueueNode<T, P> {
    fn clone(&self) -> Self {
        Self {
            internal: self.internal.clone(),
        }
    }
}

fn parent_index(index: usize) -> usize {
    (index - 1) / 2
}

fn left_child_index(index: usize) -> usize {
    2 * index + 1
}

fn is_left(index: usize) -> bool {
    index == left_child_index(parent_index(index))
}

impl<T: Eq + Hash + Clone, P: Ord + Clone> PriorityQueueInternal<T, P> {
    fn heapify(&mut self, index: usize) {
        todo!()
    }
}

impl<T: Eq + Hash + Clone, P: Ord + Clone> PriorityQueue<T, P> {
    pub fn new() -> PriorityQueue<T, P> {
        PriorityQueue {
            internal: Rc::new(RefCell::new(PriorityQueueInternal {
                nodes: Vec::new(),
                map: HashMap::new(),
            })),
        }
    }

    pub fn pop_by_priority(&mut self) -> Option<(T, P)> {
        let mut this = self.internal.borrow_mut();
        if this.nodes.len() == 0 {
            None
        } else if this.nodes.len() == 1 {
            let root = this.nodes.pop().unwrap();
            let root_node = root.internal.borrow();
            this.map.remove(&root_node.data);
            Some((root_node.data.clone(), root_node.priority.clone()))
        } else {
            let result = {
                let root = this.nodes[0].clone();
                let root_node = root.internal.borrow();
                this.map.remove(&root_node.data);
                Some((root_node.data.clone(), root_node.priority.clone()))
            };
            let last = this.nodes.pop().unwrap();
            let last_node = last.internal.borrow_mut();
            if is_left(last_node.index) {
                last_node
                    .parent
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .borrow_mut()
                    .left = None;
            } else {
                last_node
                    .parent
                    .clone()
                    .unwrap()
                    .upgrade()
                    .unwrap()
                    .borrow_mut()
                    .right = None;
            }
            if let Some(left) = last_node.left.clone() {
                let mut left_node = left.borrow_mut();
                left_node.parent = Some(Rc::downgrade(&last.internal));
                if let Some(right) = last_node.right.clone() {
                    let mut right_node = right.borrow_mut();
                    right_node.parent = Some(Rc::downgrade(&last.internal));
                }
            }
            this.nodes[0] = last.clone();
            this.heapify(0);
            result
        }
    }

    pub fn get_node_by_data(&mut self, data: &T) -> Option<PriorityQueueNode<T, P>> {
        let this = self.internal.borrow();
        this.map.get(&data).map(|reference| reference.clone())
    }

    pub fn push(&mut self, data: T, priority: P) -> PriorityQueueNode<T, P> {
        let mut this = self.internal.borrow_mut();
        if let Some(node) = this.map.get_mut(&data) {
            node.set_priority(priority);
            node.clone()
        } else {
            let index = this.nodes.len();
            if index == 0 {
                let last = PriorityQueueNode {
                    internal: Rc::new(RefCell::new(PriorityQueueNodeInternal {
                        of: Rc::downgrade(&self.internal),
                        parent: None,
                        left: None,
                        right: None,
                        index,
                        priority,
                        data,
                    })),
                };
                this.nodes.push(last.clone());
                this.map
                    .insert(last.internal.borrow().data.clone(), last.clone());
                last
            } else {
                let last = PriorityQueueNode {
                    internal: Rc::new(RefCell::new(PriorityQueueNodeInternal {
                        of: Rc::downgrade(&self.internal),
                        parent: Some(Rc::downgrade(&this.nodes[parent_index(index)].internal)),
                        left: None,
                        right: None,
                        index,
                        priority,
                        data,
                    })),
                };
                if is_left(index) {
                    last.internal
                        .borrow()
                        .parent
                        .clone()
                        .unwrap()
                        .upgrade()
                        .unwrap()
                        .borrow_mut()
                        .left = Some(last.internal.clone());
                } else {
                    last.internal
                        .borrow()
                        .parent
                        .clone()
                        .unwrap()
                        .upgrade()
                        .unwrap()
                        .borrow_mut()
                        .right = Some(last.internal.clone());
                }
                this.nodes.push(last.clone());
                this.heapify(index);
                this.map
                    .insert(last.internal.borrow().data.clone(), last.clone());
                last
            }
        }
    }
}

impl<T: Eq + Hash + Clone, P: Ord + Clone> PriorityQueueNode<T, P> {
    pub fn set_priority(&mut self, priority: P) {
        let (index, of) = {
            let mut borrow = self.internal.borrow_mut();
            borrow.priority = priority;
            (borrow.index, borrow.of.upgrade().unwrap())
        };
        let mut this = of.borrow_mut();
        this.heapify(index)
    }
}
