use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use graph::{GraphEdge, GraphNode};

pub mod graph;

pub fn dijkstra<T>(from: GraphNode<T>, to: GraphNode<T>) -> Option<Vec<GraphEdge<T>>> {
    struct HeapNode<T> {
        node: GraphNode<T>,
        distance: usize,
    }

    impl<T> PartialEq for HeapNode<T> {
        fn eq(&self, other: &Self) -> bool {
            self.distance == other.distance && self.node.eq(&other.node)
        }
    }

    impl<T> Eq for HeapNode<T> {}

    impl<T> PartialOrd for HeapNode<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<T> Ord for HeapNode<T> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            let result = self.distance.cmp(&other.distance);
            if result == Ordering::Equal {
                self.node.cmp(&other.node)
            } else {
                result
            }
        }
    }

    let mut to_visit = BinaryHeap::<HeapNode<T>>::new();
    let mut visited = HashMap::<GraphNode<T>, (usize, Vec<GraphEdge<T>>)>::new();

    visited.insert(from.clone(), (0, vec![]));
    for edge in from.adjacent_nodes() {
        to_visit.push(HeapNode {
            node: edge.to,
            distance: edge.distance,
        })
    }

    loop {
        if let Some(HeapNode { node, distance }) = to_visit.pop() {
            //
        } else {
            return None;
        }
    }
}
