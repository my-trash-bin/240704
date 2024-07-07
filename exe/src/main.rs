use std::{env, error::Error};

use data::{parse_data, Station};
use my_trash_bin_240704_lib::{
    dijkstra,
    graph::{Graph, GraphDistanceF32, GraphNode},
};

mod data;

fn find_node(
    graph: &Graph<Station, GraphDistanceF32>,
    id: &String,
) -> Option<GraphNode<Station, GraphDistanceF32>> {
    for index in 0..graph.length() {
        let node = graph[index].clone();
        if node.value().has_id(id) {
            return Some(node);
        }
    }
    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = env::args().collect::<Vec<_>>();

    let data: String = std::fs::read_to_string("data.json")?;
    // let data: String = std::fs::read_to_string(args[1])?;
    let data = parse_data(data.as_bytes())?;

    let start = find_node(&data.graph, &"에버라인선_015".to_string()).unwrap();
    let end = find_node(&data.graph, &"5호선_020".to_string()).unwrap();

    println!("{:?}", dijkstra(start, end));

    Ok(())
}
