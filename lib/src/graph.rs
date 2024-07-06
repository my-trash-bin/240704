use std::{
    cell::RefCell,
    error::Error,
    hash::{Hash, Hasher},
    io,
    ops::Deref,
    rc::{Rc, Weak},
};

pub struct Graph<T> {
    nodes: Vec<Rc<RefCell<GraphNodeInternal<T>>>>,
}

struct GraphNodeInternal<T> {
    adjacent_nodes: Vec<GraphEdgeInternal<T>>,
    data: T,
}

pub struct GraphNode<T> {
    internal: Rc<RefCell<GraphNodeInternal<T>>>,
}

impl<T> Clone for GraphNode<T> {
    fn clone(&self) -> Self {
        Self {
            internal: self.internal.clone(),
        }
    }
}

struct GraphEdgeInternal<T> {
    pub from: Weak<RefCell<GraphNodeInternal<T>>>,
    pub to: Weak<RefCell<GraphNodeInternal<T>>>,
    pub distance: usize,
}

impl<T> Clone for GraphEdgeInternal<T> {
    fn clone(&self) -> Self {
        Self {
            from: self.from.clone(),
            to: self.to.clone(),
            distance: self.distance,
        }
    }
}

impl<T> GraphEdgeInternal<T> {
    fn to_graph_edge(&self) -> GraphEdge<T> {
        GraphEdge {
            from: GraphNode {
                internal: self.from.upgrade().unwrap(),
            },
            to: GraphNode {
                internal: self.to.upgrade().unwrap(),
            },
            distance: self.distance,
        }
    }
}

pub struct GraphEdge<T> {
    pub from: GraphNode<T>,
    pub to: GraphNode<T>,
    pub distance: usize,
}

impl<T> Clone for GraphEdge<T> {
    fn clone(&self) -> Self {
        Self {
            from: self.from.clone(),
            to: self.to.clone(),
            distance: self.distance.clone(),
        }
    }
}

impl<T> Graph<T> {
    pub fn new(
        values: Vec<T>,
        adjacent_matrix: Vec<Vec<Option<usize>>>,
    ) -> Result<Graph<T>, Box<dyn Error>> {
        let length = values.len();
        let nodes: Vec<Rc<RefCell<GraphNodeInternal<T>>>> = values
            .into_iter()
            .map(|x| {
                Rc::new(RefCell::new(GraphNodeInternal {
                    data: x,
                    adjacent_nodes: vec![],
                }))
            })
            .collect();

        if adjacent_matrix.len() != length || adjacent_matrix.iter().any(|x| x.len() != length) {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Adjacent matrix size mismatch",
            )));
        }
        if adjacent_matrix.iter().enumerate().any(
            |(i, l)| {
                if let Some(z) = l[i] {
                    z != 0
                } else {
                    false
                }
            },
        ) {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Adjacent matrix should have zeroes on its diagonal",
            )));
        }

        for i in 0..length {
            for j in 0..length {
                if i == j {
                    continue;
                }
                if let Some(distance) = adjacent_matrix[i][j] {
                    let from = Rc::downgrade(&Rc::clone(&nodes[i]));
                    let to = Rc::downgrade(&Rc::clone(&nodes[i]));
                    nodes[i]
                        .borrow_mut()
                        .adjacent_nodes
                        .push(GraphEdgeInternal { from, to, distance });
                }
            }
        }

        Ok(Graph { nodes })
    }

    pub fn length(&self) -> usize {
        self.nodes.len()
    }
}

impl<T> Deref for GraphNodeInternal<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

pub struct GraphNodeAdjacent<T> {
    pub nodes: Vec<GraphEdge<T>>,
}

impl<T> Clone for GraphNodeAdjacent<T> {
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.clone(),
        }
    }
}

impl<T> GraphNode<T> {
    pub fn adjacent(&self) -> GraphNodeAdjacent<T> {
        GraphNodeAdjacent {
            nodes: self
                .internal
                .borrow()
                .adjacent_nodes
                .iter()
                .map(|internal| GraphEdge {
                    from: GraphNode {
                        internal: internal.from.upgrade().unwrap(),
                    },
                    to: GraphNode {
                        internal: internal.to.upgrade().unwrap(),
                    },
                    distance: internal.distance,
                })
                .collect(),
        }
    }
}

impl<T> PartialEq for GraphNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.internal.as_ptr() == other.internal.as_ptr()
    }
}

impl<T> Eq for GraphNode<T> {}

impl<T> PartialOrd for GraphNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for GraphNode<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.internal.as_ptr().cmp(&other.internal.as_ptr())
    }
}

impl<T> Hash for GraphNode<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.internal.as_ptr().hash(state);
    }
}
