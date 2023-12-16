use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::path::Path;
use petgraph::graph::UnGraph;
use std::collections::HashMap;

/// Reads the dataset
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the dataset file.
///
/// # Returns
///
/// A result containing the graph or an I/O error.
pub fn read_dataset(file_path: &str) -> io::Result<UnGraph<u32, ()>> {
    let path = Path::new(file_path);
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut graph = UnGraph::<u32, ()>::new_undirected();
    let mut node_indices = HashMap::new();

    for line in reader.lines() {
        let line = line?;
        if !line.starts_with("#") && !line.is_empty() {
            let nodes: Vec<&str> = line.split_whitespace().collect();
            if nodes.len() == 2 {
                let from = nodes[0].parse::<u32>().unwrap();
                let to = nodes[1].parse::<u32>().unwrap();

                // Ensure each node is uniquely represented in the graph
                let from_index = *node_indices.entry(from).or_insert_with(|| graph.add_node(from));
                let to_index = *node_indices.entry(to).or_insert_with(|| graph.add_node(to));

                // Add an edge between the nodes
                graph.add_edge(from_index, to_index, ());
            }
        }
    }

    Ok(graph)
    
}