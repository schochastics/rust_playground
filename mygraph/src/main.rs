mod centrality;
mod graph;
mod utils;
use crate::graph::Graph;
use log::info;
use std::collections::{HashSet, VecDeque};

// fn dijkstra(graph: &Graph, src: usize) -> Vec<usize> {
//     let mut distances = vec![usize::MAX; graph.vertices];
//     let mut queue = VecDeque::new();
//     let mut visited = HashSet::new();

//     distances[src] = 0;
//     queue.push_back(src);
//     visited.insert(src);

//     while let Some(u) = queue.pop_front() {
//         for &v in &graph.adj_list[u] {
//             if !visited.contains(&v) {
//                 visited.insert(v);
//                 queue.push_back(v);
//                 distances[v] = distances[u] + 1; // Since each edge cost is considered 1
//             }
//         }
//     }

//     distances
// }

// // centrality and shortest path
// impl Graph {
//     fn distances(&self) -> Vec<Vec<usize>> {
//         (0..self.vertices).map(|src| dijkstra(&self, src)).collect()
//     }
// }

fn main() {
    env_logger::init();

    // let mut graph = Graph::new(5);

    // graph.add_edge(0, 1);
    // graph.add_edge(0, 4);
    // graph.add_edge(1, 2);
    // graph.add_edge(1, 3);
    // graph.add_edge(1, 4);
    // graph.add_edge(2, 3);
    // graph.add_edge(3, 4);

    let graph = utils::read_edgelist("examples/gnp1000.csv").expect("error");
    // centrality
    println!("Degree");
    let dc = time(|| graph.degree());
    println!("Closeness");
    let cc = time(|| graph.closeness_centrality());
    println!("Betweenness");
    let bc = time(|| graph.betweenness_centrality());
    println!("Eigenvector");
    let ec = time(|| graph.eigenvector_centrality());
}

fn time<T, F: FnOnce() -> T>(f: F) -> T {
    let start = std::time::Instant::now();
    let res = f();
    info!("Execution took {:?}", start.elapsed());
    res
}
