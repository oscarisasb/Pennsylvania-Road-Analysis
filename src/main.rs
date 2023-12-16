mod construction;
mod analysis;

use std::io;
use construction::read_dataset;
use analysis::*;

/// Main function to run the analyses on the road network dataset.
fn main() -> io::Result<()> {
    let file_path = "C:\\Users\\Oscarisasb\\Desktop\\roadNet-PA.txt";
    let graph = read_dataset(file_path)?;

    println!("Total number of nodes: {}", graph.node_count());

    // Perform and display various analyses on the graph
    find_total_nodes_with_more_than_four_connections(&graph);
    let (distance_1_count, distance_2_count) = count_nodes_with_distance_criteria(&graph);
    println!("Number of nodes meeting the distance-1 connection criteria: {}", distance_1_count);
    println!("Number of nodes meeting the distance-2 connection criteria: {}", distance_2_count);
    analyze_connected_components(&graph);
    analyze_degree_distribution(&graph);
    shortest_path_from_node(&graph, 456, 3); // Enter the node number
    
    Ok(())
}
