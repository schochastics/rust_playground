use csv::Reader;
use std::error::Error;
use std::fs::File;

use crate::graph::Graph;

pub fn read_edgelist(file_path: &str) -> Result<Graph, Box<dyn Error>> {
    let mut reader = Reader::from_reader(File::open(file_path)?);
    let mut edgelist: Vec<(usize, usize)> = Vec::new();
    let mut max_node_index = 0;
    for result in reader.records() {
        let record = result?;
        let src: usize = record[0].parse()?;
        let dest: usize = record[1].parse()?;
        max_node_index = max_node_index.max(src).max(dest);
        edgelist.push((src, dest));
    }

    // Assuming node indices start from 0 and are continuous
    let vertices = max_node_index + 1;
    let graph = Graph::from_edgelist(edgelist, vertices);

    Ok(graph)
}
