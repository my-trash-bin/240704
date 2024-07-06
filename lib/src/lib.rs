use std::{
    cmp::min_by_key,
    collections::{BinaryHeap, HashMap},
    fmt::Debug,
};

use graph::{GraphDistance, GraphEdge, GraphNode};

pub mod graph;

pub fn dijkstra<T: Debug, D: GraphDistance + Debug>(
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
    println!("visited: {:?}", from);
    println!("to_visit: {:?}", to_visit);

    while let Some(node_to_visit) = to_visit.pop() {
        println!("visiting: {:?}", node_to_visit);
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
                    println!("one of edge to node to visit is already visited");
                    Some((
                        total_distance.clone() + current.distance.clone(),
                        GraphEdge {
                            from: if let Some(GraphEdge { to, .. }) = last_move {
                                to.clone()
                            } else {
                                from.clone()
                            },
                            to: node_to_visit.clone(),
                            distance: current.distance,
                        },
                    ))
                } else {
                    println!("one of edge to node to visit is not already visited");
                    None
                };

                let result = match (min, current) {
                    (Some(min), Some(current)) => Some(min_by_key(min, current, |d| d.0.clone())),
                    (Some(min), None) => Some(min),
                    (None, Some(current)) => Some(current),
                    (None, None) => None,
                };
                println!("  so, returning {:?}", result);
                result
            })
            .unwrap();

        if let Some(MapNode { total_distance, .. }) = visited.get(&node_to_visit) {
            // visit only if found shorter distance if already visited
            if new_distance < *total_distance {
                visited.insert(
                    node_to_visit.clone(),
                    MapNode {
                        total_distance: new_distance,
                        last_move: Some(new_move.clone()),
                    },
                );
                for edge in node_to_visit.adjacent().nodes {
                    to_visit.push(edge.to)
                }
                println!("visited: {:?}", node_to_visit);
                println!("to_visit: {:?}", to_visit);
            } else {
                println!("not needed to visit.");
            }
        } else {
            // unconditionally visit if not already visited
            visited.insert(
                node_to_visit.clone(),
                MapNode {
                    total_distance: new_distance,
                    last_move: Some(new_move),
                },
            );
            for edge in node_to_visit.adjacent().nodes {
                to_visit.push(edge.to)
            }
            println!("visited: {:?}", node_to_visit);
            println!("to_visit: {:?}", to_visit);
        }
    }
    // visited all connected
    println!("visited all connected nodes.");
    return if let Some(MapNode { last_move, .. }) = visited.get(&to) {
        // from and to is connected
        Some(if let Some(last_move) = last_move {
            let mut last_move = Some(last_move.clone());
            let mut result = Vec::new();
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
