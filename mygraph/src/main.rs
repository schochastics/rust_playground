mod centrality;
mod graph;
mod structural;
mod utils;
use std::env;

fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    let path = &args[1];
    // let mut graph = Graph::new(5);

    // graph.add_edge(0, 1);
    // graph.add_edge(0, 4);
    // graph.add_edge(1, 2);
    // graph.add_edge(1, 3);
    // graph.add_edge(1, 4);
    // graph.add_edge(2, 3);
    // graph.add_edge(3, 4);

    let graph = utils::read_edgelist(path).expect("error");
    println!("to sparse Matrix");
    time(|| graph.to_adjacency_matrix_sparse());
    // centrality
    println!("Degree");
    time(|| graph.degree());
    println!("Closeness");
    time(|| graph.closeness_centrality());
    println!("Betweenness");
    time(|| graph.betweenness_centrality());
    println!("Eigenvector");
    time(|| graph.eigenvector_centrality());
    println!("Triangles");
    time(|| graph.count_triangles());
}

fn time<T, F: FnOnce() -> T>(f: F) -> T {
    let start = std::time::Instant::now();
    let res = f();
    println!("Execution took {:?}", start.elapsed());
    res
}
