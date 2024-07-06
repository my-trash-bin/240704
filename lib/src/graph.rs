use std::{
    cell::RefCell,
    error::Error,
    hash::{Hash, Hasher},
    io,
    ops::{Add, Deref},
    rc::{Rc, Weak},
};

pub trait GraphDistance: Ord + Add<Output = Self> + Clone + Sized {
    fn zero() -> Self;
}
macro_rules! impl_graph_distance {
    ($id: ident) => {
        impl GraphDistance for $id {
            fn zero() -> Self {
                0
            }
        }
    };
}
impl_graph_distance!(u8);
impl_graph_distance!(u16);
impl_graph_distance!(u32);
impl_graph_distance!(u64);
impl_graph_distance!(u128);
impl_graph_distance!(usize);
impl_graph_distance!(i8);
impl_graph_distance!(i16);
impl_graph_distance!(i32);
impl_graph_distance!(i64);
impl_graph_distance!(i128);
impl_graph_distance!(isize);

pub struct Graph<T, D: GraphDistance> {
    nodes: Vec<Rc<RefCell<GraphNodeInternal<T, D>>>>,
}

struct GraphNodeInternal<T, D: GraphDistance> {
    adjacent_nodes: Vec<GraphEdgeInternal<T, D>>,
    data: T,
}

pub struct GraphNode<T, D: GraphDistance> {
    internal: Rc<RefCell<GraphNodeInternal<T, D>>>,
}

impl<T, D: GraphDistance> Clone for GraphNode<T, D> {
    fn clone(&self) -> Self {
        Self {
            internal: self.internal.clone(),
        }
    }
}

struct GraphEdgeInternal<T, D: GraphDistance> {
    pub from: Weak<RefCell<GraphNodeInternal<T, D>>>,
    pub to: Weak<RefCell<GraphNodeInternal<T, D>>>,
    pub distance: D,
}

impl<T, D: GraphDistance> Clone for GraphEdgeInternal<T, D> {
    fn clone(&self) -> Self {
        Self {
            from: self.from.clone(),
            to: self.to.clone(),
            distance: self.distance.clone(),
        }
    }
}

impl<T, D: GraphDistance> GraphEdgeInternal<T, D> {
    fn to_graph_edge(&self) -> GraphEdge<T, D> {
        GraphEdge {
            from: GraphNode {
                internal: self.from.upgrade().unwrap(),
            },
            to: GraphNode {
                internal: self.to.upgrade().unwrap(),
            },
            distance: self.distance.clone(),
        }
    }
}

pub struct GraphEdge<T, D: GraphDistance> {
    pub from: GraphNode<T, D>,
    pub to: GraphNode<T, D>,
    pub distance: D,
}

impl<T, D: GraphDistance> Clone for GraphEdge<T, D> {
    fn clone(&self) -> Self {
        Self {
            from: self.from.clone(),
            to: self.to.clone(),
            distance: self.distance.clone(),
        }
    }
}

impl<T, D: GraphDistance> Graph<T, D> {
    pub fn new(
        values: Vec<T>,
        adjacent_matrix: Vec<Vec<Option<D>>>,
    ) -> Result<Graph<T, D>, Box<dyn Error>> {
        let length = values.len();
        let nodes: Vec<Rc<RefCell<GraphNodeInternal<T, D>>>> = values
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
        if adjacent_matrix.iter().enumerate().any(|(i, l)| {
            if let Some(z) = l[i].clone() {
                z != D::zero()
            } else {
                false
            }
        }) {
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
                if let Some(distance) = adjacent_matrix[i][j].clone() {
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

impl<T, D: GraphDistance> Deref for GraphNodeInternal<T, D> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.data
    }
}

pub struct GraphNodeAdjacent<T, D: GraphDistance> {
    pub nodes: Vec<GraphEdge<T, D>>,
}

impl<T, D: GraphDistance> Clone for GraphNodeAdjacent<T, D> {
    fn clone(&self) -> Self {
        Self {
            nodes: self.nodes.clone(),
        }
    }
}

impl<T, D: GraphDistance> GraphNode<T, D> {
    pub fn adjacent(&self) -> GraphNodeAdjacent<T, D> {
        GraphNodeAdjacent {
            nodes: self
                .internal
                .borrow()
                .adjacent_nodes
                .iter()
                .map(GraphEdgeInternal::to_graph_edge)
                .collect(),
        }
    }
}

impl<T, D: GraphDistance> PartialEq for GraphNode<T, D> {
    fn eq(&self, other: &Self) -> bool {
        self.internal.as_ptr() == other.internal.as_ptr()
    }
}

impl<T, D: GraphDistance> Eq for GraphNode<T, D> {}

impl<T, D: GraphDistance> PartialOrd for GraphNode<T, D> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, D: GraphDistance> Ord for GraphNode<T, D> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.internal.as_ptr().cmp(&other.internal.as_ptr())
    }
}

impl<T, D: GraphDistance> Hash for GraphNode<T, D> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.internal.as_ptr().hash(state);
    }
}
