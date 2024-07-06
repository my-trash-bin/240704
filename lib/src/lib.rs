use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use graph::{GraphEdge, GraphNode};

pub mod graph;

pub fn dijkstra<T>(from: GraphNode<T>, to: GraphNode<T>) -> Option<Vec<GraphEdge<T>>> {
    struct HeapNode<T> {
        edge: GraphEdge<T>,
        total_distance: usize,
    }

    impl<T> PartialEq for HeapNode<T> {
        fn eq(&self, other: &Self) -> bool {
            self.edge.to.eq(&other.edge.to)
            // from, distance, total_distance is intentionally missing
        }
    }

    impl<T> Eq for HeapNode<T> {}

    impl<T> PartialOrd for HeapNode<T> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<T> Ord for HeapNode<T> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.edge.to.cmp(&other.edge.to)
            // from, distance, total_distance is intentionally missing
        }
    }

    let mut to_visit = BinaryHeap::<HeapNode<T>>::new();
    let mut visited = HashMap::<GraphNode<T>, (usize, Vec<GraphEdge<T>>)>::new();

    visited.insert(from.clone(), (0, vec![]));
    for edge in from.adjacent().nodes {
        to_visit.push(HeapNode {
            edge: edge.clone(),
            total_distance: edge.distance,
        })
    }

    loop {
        if let Some(HeapNode {
            edge,
            total_distance,
        }) = to_visit.pop()
        {
            if let Some((previous_distance, path)) = visited.get(&edge.to) {
                if edge.distance < *previous_distance {
                    let mut path = visited.get(&from).unwrap().1.clone();
                    path.push(GraphEdge {})
                }
            }
        } else {
            return None;
        }
    }
}
