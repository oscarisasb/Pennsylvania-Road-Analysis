use petgraph::algo::{connected_components, dijkstra};
use petgraph::graph::{UnGraph, NodeIndex};
use std::collections::HashMap;
use std::collections::HashSet;

/// Analyzes and prints the number of connected components in the graph.
///
/// # Arguments
///
/// * `graph` - A reference to the graph to be analyzed.
pub fn analyze_connected_components(graph: &UnGraph<u32, ()>) {
    let components = connected_components(graph);
    println!("Number of connected components: {}", components);
}

/// Finds and prints the shortest paths from a given node to all others within a maximum distance.
///
/// # Arguments
///
/// * `graph` - A reference to the graph.
/// * `start_node` - The starting node for path calculations.
/// * `max_distance` - The maximum distance of the paths to be considered.
pub fn shortest_path_from_node(graph: &UnGraph<u32, ()>, start_node: u32, max_distance: u32) {
    let start_index = NodeIndex::new(start_node as usize);
    let path_map = dijkstra(graph, start_index, None, |_| 1);

    for (node, cost) in path_map {
        if cost <= max_distance {
            println!("Distance from node {} to node {} is {}", start_node, node.index(), cost);
        }
    }
}

/// Analyzes and prints the degree distribution of nodes in the graph.
///
/// # Arguments
///
/// * `graph` - A reference to the graph.
pub fn analyze_degree_distribution(graph: &UnGraph<u32, ()>) {
    let mut degree_count = HashMap::new();
    for node in graph.node_indices() {
        let degree = graph.edges(node).count();
        *degree_count.entry(degree).or_insert(0) += 1;
    }

    println!("Degree Distribution:");
    for (degree, count) in degree_count.iter() {
        println!("Degree {}: {} nodes", degree, count);
    }
}

/// Finds and prints the total number of nodes with more than four connections.
///
/// # Arguments
///
/// * `graph` - A reference to the graph.
pub fn find_total_nodes_with_more_than_four_connections(graph: &UnGraph<u32, ()>) {
    let mut total_nodes = 0;

    for node in graph.node_indices() {
        let degree = graph.edges(node).count();
        if degree > 4 {
            total_nodes += 1;
        }
    }

    println!("Total number of nodes with more than four connections: {}", total_nodes);
}

/// Counts nodes with connections at distance 1 and 2 meeting specific criteria.
///
/// # Arguments
///
/// * `graph` - A reference to the graph.
///
/// # Returns
///
/// A tuple containing counts for distance 1 and distance 2 criteria.
pub fn count_nodes_with_distance_criteria(graph: &UnGraph<u32, ()>) -> (usize, usize) {
    let mut distance_1_count = 0;
    let mut distance_2_count = 0;

    for node in graph.node_indices() {
        let node_index = node.index() as i64;
        let mut neighbors: Vec<_> = Vec::new();
        let mut counted_for_1_distance = HashSet::new();

        // Check direct neighbors (distance 1)
        for neighbor in graph.neighbors(node) {
            let neighbor_index = neighbor.index() as i64;
            neighbors.push(neighbor_index);

            if (neighbor_index - node_index >= 5000 || node_index - neighbor_index >= 5000) && !counted_for_1_distance.contains(&neighbor_index) {
                distance_1_count += 1;
                counted_for_1_distance.insert(neighbor_index);
            }
        }

        // Check neighbors of neighbors (distance 2)
        for &neighbor in &neighbors {
            for neighbor_of_neighbor in graph.neighbors(NodeIndex::new(neighbor as usize)) {
                let neighbor_of_neighbor_index = neighbor_of_neighbor.index() as i64;
                if !neighbors.contains(&neighbor_of_neighbor_index) &&
                   (neighbor_of_neighbor_index - node_index >= 5000 || node_index - neighbor_of_neighbor_index >= 5000) &&
                   !counted_for_1_distance.contains(&neighbor_of_neighbor_index) {
                    distance_2_count += 1;
                    break; // Break to avoid double counting for this node
                }
            }
        }
    }

    (distance_1_count, distance_2_count)
}

///Tests
#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::UnGraph;

    fn create_test_graph() -> UnGraph<u32, ()> {
        let mut graph = UnGraph::new_undirected();
        let n1 = graph.add_node(1);
        let n2 = graph.add_node(2);
        let n3 = graph.add_node(3);
        let n4 = graph.add_node(4);
        let n5 = graph.add_node(5);

        graph.add_edge(n1, n2, ());
        graph.add_edge(n2, n3, ());
        graph.add_edge(n3, n4, ());
        graph.add_edge(n4, n5, ());
        graph.add_edge(n5, n1, ());
        graph.add_edge(n3, n5, ());

        graph
    }

    #[test]
    fn test_analyze_connected_components() {
        let graph = create_test_graph();
        let components = connected_components(&graph);
        assert_eq!(components, 1);
    }

    #[test]
    fn test_find_total_nodes_with_more_than_four_connections() {
        let graph = create_test_graph();
        let count = find_total_nodes_with_more_than_four_connections(&graph);
        assert_eq!(count,());
    }

    #[test]
    fn test_shortest_path_from_node() {
        let graph = create_test_graph();
        let start_node = 1; // Assuming this node exists in test graph
        let start_index = NodeIndex::new(start_node as usize);
        let path_map = dijkstra(&graph, start_index, None, |_| 1);

        // Assuming know the expected shortest path lengths from node 1
        assert_eq!(path_map[&NodeIndex::new(2)], 1); // Example: path length from node 1 to node 2
        assert_eq!(path_map[&NodeIndex::new(3)], 2); // Example: path length from node 1 to node 3
    
    }

    #[test]
    fn test_analyze_degree_distribution() {
        let graph = create_test_graph();
        let mut degree_count = HashMap::new();
        for node in graph.node_indices() {
            let degree = graph.edges(node).count();
            *degree_count.entry(degree).or_insert(0) += 1;
        }

        // Assuming  knowing the expected degree distribution in your test graph
        assert_eq!(degree_count[&2], 3); // Example: 3 nodes with degree 2
        assert_eq!(degree_count[&3], 2); // Example: 2 nodes with degree 3
       
    }

}