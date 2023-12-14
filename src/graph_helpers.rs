use petgraph::Graph;
use petgraph::graph::NodeIndex;
use std::collections::HashMap;

// A public function for the minimum path in a graph
pub fn find_minimum_path(

    graph: &Graph<String, ()>, 
    city_mappings: &HashMap<String, NodeIndex>, 
    start: &str, 
    target: &str
    
) -> Option<usize> {

    // Get the NodeIndex of the starting city from the hashmap
    let start_node = city_mappings.get(start)?;

    // Get the target city
    let target_node = city_mappings.get(target)?;

    // Use Dijkstra's algorithm to find the shortest paths from the start node to all other nodes
    let shortest_paths = petgraph::algo::dijkstra(graph, *start_node, Some(*target_node), |_| 1);

    // Get the length of the shortest path to the target node and clone 
    shortest_paths.get(target_node).cloned()
}