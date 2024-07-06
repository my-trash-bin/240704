use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

use graph::{GraphDistance, GraphEdge, GraphNode};

pub mod graph;

pub fn dijkstra<T, D: GraphDistance>(
    from: GraphNode<T, D>,
    to: GraphNode<T, D>,
) -> Option<Vec<GraphEdge<T, D>>> {
    struct HeapNode<T, D: GraphDistance> {
        edge: GraphEdge<T, D>,
        total_distance: D,
    }

    impl<T, D: GraphDistance> PartialEq for HeapNode<T, D> {
        fn eq(&self, other: &Self) -> bool {
            self.edge.to.eq(&other.edge.to)
            // from, distance, total_distance is intentionally missing
        }
    }

    impl<T, D: GraphDistance> Eq for HeapNode<T, D> {}

    impl<T, D: GraphDistance> PartialOrd for HeapNode<T, D> {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<T, D: GraphDistance> Ord for HeapNode<T, D> {
        fn cmp(&self, other: &Self) -> Ordering {
            self.edge.to.cmp(&other.edge.to)
            // from, distance, total_distance is intentionally missing
        }
    }

    let mut to_visit = BinaryHeap::<HeapNode<T, D>>::new();
    let mut visited = HashMap::<GraphNode<T, D>, (D, Option<GraphEdge<T, D>>)>::new();

    visited.insert(from.clone(), (D::zero(), None));
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
            if let Some((previous_distance, _)) = visited.get(&edge.to) {
                let new_distance = total_distance + edge.distance.clone();
                if new_distance < *previous_distance {
                    visited.insert(edge.to.clone(), (new_distance, Some(edge)));
                }
            }
        } else {
            return None;
        }
    }
}
