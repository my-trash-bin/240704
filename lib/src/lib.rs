use std::{
    cmp::min_by_key,
    collections::{BinaryHeap, HashMap},
};

use graph::{GraphDistance, GraphEdge, GraphNode};

pub mod graph;

pub fn dijkstra<T, D: GraphDistance>(
    from: GraphNode<T, D>,
    to: GraphNode<T, D>,
) -> Option<Vec<GraphEdge<T, D>>> {
    struct MapNode<T, D: GraphDistance> {
        total_distance: D,
        last_move: Option<GraphEdge<T, D>>,
    }

    let mut to_visit = BinaryHeap::<GraphNode<T, D>>::new();
    let mut visited = HashMap::<GraphNode<T, D>, MapNode<T, D>>::new();

    // initial visit
    visited.insert(
        from.clone(),
        MapNode {
            total_distance: D::zero(),
            last_move: None,
        },
    );
    for edge in from.adjacent().nodes {
        to_visit.push(edge.to)
    }

    loop {
        if let Some(node_to_visit) = to_visit.pop() {
            if let Some(MapNode { total_distance, .. }) = visited.get(&node_to_visit) {
                // get new distance
                let (new_distance, new_move) = node_to_visit
                    .reverse_adjacent()
                    .nodes
                    .into_iter()
                    .fold(None, |min, current| {
                        let current = if let Some(MapNode {
                            total_distance,
                            last_move,
                        }) = visited.get(&current.from)
                        {
                            Some((total_distance.clone() + current.distance, last_move))
                        } else {
                            None
                        };

                        min_by_key(min, current, |d| d.clone().map(|d| d.0))
                    })
                    .unwrap();

                // visit
                if new_distance < *total_distance {
                    visited.insert(
                        node_to_visit.clone(),
                        MapNode {
                            total_distance: new_distance,
                            last_move: new_move.clone(),
                        },
                    );
                    for edge in node_to_visit.adjacent().nodes {
                        to_visit.push(edge.to)
                    }
                }
            }
        } else {
            // if visited all connected
            return if let Some(MapNode { last_move, .. }) = visited.get(&to) {
                Some(if let Some(last_move) = last_move {
                    let mut result = vec![last_move.clone()];
                    let mut last_move = Some(last_move.clone());
                    while let Some(edge) = last_move {
                        result.push(edge.clone());
                        last_move = visited.get(&edge.from).unwrap().last_move.clone();
                    }
                    result.reverse();
                    result
                } else {
                    vec![]
                })
            } else {
                // from and to is not connected
                None
            };
        }
    }
}

#[cfg(test)]
mod tests {
    use graph::{Graph, GraphDistanceF32};

    use super::*;

    #[test]
    fn should_work() {
        let adjacent_matrix = vec![
            vec![None, Some(GraphDistanceF32(1f32))],
            vec![Some(GraphDistanceF32(2f32)), None],
        ];
        let values = vec!["Hello", "World"];
        let graph = Graph::new(values, adjacent_matrix).unwrap();

        let hello = graph[0].clone();
        let world = graph[1].clone();

        assert_eq!(
            dijkstra(hello.clone(), world.clone()),
            Some(vec![GraphEdge {
                distance: GraphDistanceF32(1f32),
                from: hello.clone(),
                to: world.clone()
            }])
        );

        assert_eq!(
            dijkstra(world.clone(), hello.clone()),
            Some(vec![GraphEdge {
                distance: GraphDistanceF32(2f32),
                from: world.clone(),
                to: hello.clone()
            }])
        );

        assert_eq!(dijkstra(hello.clone(), hello.clone()), Some(vec![]));
    }
}
