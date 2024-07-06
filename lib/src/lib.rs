use std::collections::HashMap;

use graph::{GraphDistance, GraphEdge, GraphNode};
use priority_queue::PriorityQueue;

pub mod graph;
pub mod priority_queue;

pub fn dijkstra<T, D: GraphDistance>(
    from: GraphNode<T, D>,
    to: GraphNode<T, D>,
) -> Option<Vec<GraphEdge<T, D>>> {
    let mut to_visit = PriorityQueue::<GraphNode<T, D>, D, GraphEdge<T, D>>::new();
    let mut visited = HashMap::<GraphNode<T, D>, Option<GraphEdge<T, D>>>::new();

    visited.insert(from.clone(), None);
    for edge in from.adjacent().nodes {
        to_visit.push(edge.to.clone(), edge.distance.clone(), edge);
    }

    while let Some((node_to_visit, new_distance, new_move)) = to_visit.pop_by_priority() {
        visited.insert(node_to_visit.clone(), Some(new_move));
        if node_to_visit == to {
            break;
        }
        for edge in node_to_visit.adjacent().nodes {
            if !visited.contains_key(&edge.to) {
                to_visit.push(
                    edge.to.clone(),
                    new_distance.clone() + edge.distance.clone(),
                    edge,
                );
            }
        }
    }

    if let Some(last_move) = visited.get(&to) {
        Some(if let Some(last_move) = last_move {
            let mut last_move = Some(last_move.clone());
            let mut result = Vec::new();
            while let Some(edge) = last_move {
                result.push(edge.clone());
                last_move = visited.get(&edge.from).unwrap().clone();
            }
            result.reverse();
            result
        } else {
            vec![]
        })
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use graph::{Graph, GraphDistanceF32};

    use super::*;

    #[test]
    fn simplest_case() {
        let adjacent_matrix = vec![
            vec![None, Some(GraphDistanceF32(1f32)), None],
            vec![Some(GraphDistanceF32(2f32)), None, None],
            vec![Some(GraphDistanceF32(3f32)), None, None],
        ];
        let values = vec!["1", "2", "3"];
        let graph = Graph::new(values, adjacent_matrix).unwrap();

        let node0 = graph[0].clone();
        let node1 = graph[1].clone();
        let node2 = graph[2].clone();

        assert_eq!(
            dijkstra(node0.clone(), node1.clone()),
            Some(vec![GraphEdge {
                distance: GraphDistanceF32(1f32),
                from: node0.clone(),
                to: node1.clone()
            }])
        );

        assert_eq!(
            dijkstra(node1.clone(), node0.clone()),
            Some(vec![GraphEdge {
                distance: GraphDistanceF32(2f32),
                from: node1.clone(),
                to: node0.clone()
            }])
        );

        assert_eq!(dijkstra(node0.clone(), node0.clone()), Some(vec![]));

        assert_eq!(dijkstra(node0.clone(), node2.clone()), None);

        assert_eq!(
            dijkstra(node2.clone(), node1.clone()),
            Some(vec![
                GraphEdge {
                    distance: GraphDistanceF32(3f32),
                    from: node2.clone(),
                    to: node0.clone()
                },
                GraphEdge {
                    distance: GraphDistanceF32(1f32),
                    from: node0.clone(),
                    to: node1.clone()
                }
            ])
        );
    }

    #[test]
    fn should_work() {
        //
    }
}
