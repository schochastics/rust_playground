mod centrality;
mod graph;
mod utils;
use log::info;

//testing area
//--------------------------------------------------------------------

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
}

fn time<T, F: FnOnce() -> T>(f: F) -> T {
    let start = std::time::Instant::now();
    let res = f();
    println!("Execution took {:?}", start.elapsed());
    res
}
