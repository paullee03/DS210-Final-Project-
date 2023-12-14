use petgraph::Graph;
use petgraph::Undirected;
use std::collections::VecDeque;
use petgraph::graph::NodeIndex;

pub struct RoadNetwork {
    // In the graph, each node is a String (representing a city) and edges are undirected
    road_graph: Graph<String, (), Undirected>,
}

impl RoadNetwork {
    pub fn new(road_graph: Graph<String, (), Undirected>) -> Self {
        RoadNetwork { road_graph }
    }

    // Breadth First Search (BFS) from an origin node to a destination node
    fn perform_bfs(&self, origin: NodeIndex, destination: NodeIndex) -> Option<usize> {

        // Return 0 is the origin and destination are equal
        if origin == destination {
            return Some(0);
        }

        let mut visited_nodes = vec![false; self.road_graph.node_count()];
        let mut node_queue = VecDeque::new();

        // Mark the origin node as visited and add it to the queue with a distance of 0
        visited_nodes[origin.index()] = true;
        node_queue.push_back((origin, 0));

        // Process nodes in the queue while its not empty
        while let Some((current_node, dist)) = node_queue.pop_front() {

            // If the current node is the destination, return the distance
            if current_node == destination {
                return Some(dist);
            }

            // Iterate over neighbors of the current node
            for adjacent in self.road_graph.neighbors(current_node) {

                if !visited_nodes[adjacent.index()] {
                    visited_nodes[adjacent.index()] = true;
                    node_queue.push_back((adjacent, dist + 1));
                }
            }
        }

        None
    }

    // Public method to evaluate degrees of separation between all pairs of nodes 
    pub fn evaluate_degrees(&self) -> Vec<usize> {
      
        let mut degree_counts = vec![0; self.road_graph.node_count() + 1];

        // Iterate over all pairs of nodes in the graph
        for start_node in self.road_graph.node_indices() {
            for end_node in self.road_graph.node_indices() {

                // Perform BFS for each pair to get the degree count
                if let Some(separation) = self.perform_bfs(start_node, end_node) {
                    degree_counts[separation] += 1;
                }
            }
        }

        degree_counts
    }

}