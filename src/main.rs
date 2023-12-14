use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::collections::HashMap;
use petgraph::Graph;
use petgraph::Undirected;

mod analysis;
mod graph_helpers;

fn main() -> io::Result<()> {
    let file = File::open("src/euroroad.csv")?;

    // Create a buffered reader for efficiency 
    let buf_reader = BufReader::new(file);

    // Initialize a new  graph where each node is a String (city name)
    let mut euro_graph = Graph::<String, (), Undirected>::new_undirected();

    // Create a hashmap to keep track of city name and city code from the graph created 
    let mut city_references: HashMap<String, _> = HashMap::new();

    // Iterate over each line and get index of two cities' connections in the graph
    for line in buf_reader.lines() {
        let line_content = line?;
        let cities = line_content.split(',').map(String::from).collect::<Vec<String>>();
        
        let (city1, city2) = (cities[0].clone(), cities[1].clone());
        let city1_index = *city_references.entry(city1.clone()).or_insert_with(|| euro_graph.add_node(city1));
        let city2_index = *city_references.entry(city2.clone()).or_insert_with(|| euro_graph.add_node(city2));

        // Add an edge between city1 and city2 in the graph
        euro_graph.add_edge(city1_index, city2_index, ());
    }

    // RoadNetwork  analysis
    let graph_analyzer = analysis::RoadNetwork::new(euro_graph);
    // Get the degrees of separation stats
    let separation_stats = graph_analyzer.evaluate_degrees();

    // Iterate through the degrees of separation statistics and print 
    for (degree, count) in separation_stats.iter().enumerate() {
        if *count > 0 {
            println!("{} degrees of separation: {} connections.", degree, count);
        }
    }

    // Statistical analysis of the separation degrees
    let total_connections: usize = separation_stats.iter().sum();
    let mean_separation: f64 = separation_stats.iter().enumerate()
        .map(|(degree, count)| degree * count)
        .sum::<usize>() as f64 / total_connections as f64;

    let variance: f64 = separation_stats.iter().enumerate()
        .map(|(degree, count)| {
            let diff = degree as f64 - mean_separation;
            diff * diff * (*count as f64) 
        })
        .sum::<f64>() / total_connections as f64;
    let standard_deviation = variance.sqrt(); 

    println!("Total connections: {}", total_connections);
    println!("Average degree of separation: {:.2}", mean_separation);
    println!("Standard deviation of degrees of separation: {:.2}", standard_deviation);

    Ok(())
}

mod tests {
    use super::*;
    use crate::analysis::RoadNetwork; 
    use petgraph::Undirected;

    // Test for evaluating degrees of separation
    #[test]
    fn test_evaluate_degrees() {

        let mut graph = Graph::<String, (), Undirected>::new_undirected();
        let city1 = graph.add_node("City1".to_string());
        let city2 = graph.add_node("City2".to_string());
        let city3 = graph.add_node("City3".to_string());
        graph.add_edge(city1, city2, ());
        graph.add_edge(city2, city3, ());

        let road_network = RoadNetwork::new(graph);
        let degrees = road_network.evaluate_degrees();

    }

// Test to verify the existence of a connection in the RoadNetwork
    #[test]
    fn test_connection_existence() {

        let mut graph = Graph::<String, (), Undirected>::new_undirected();
        let city1 = graph.add_node("City1".to_string());
        let city2 = graph.add_node("City2".to_string());
        graph.add_edge(city1, city2, ());

        // Create a RoadNetwork with the graph
        let road_network = RoadNetwork::new(graph);

    }
}