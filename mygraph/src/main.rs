use std::collections::{HashSet, VecDeque};

struct Graph {
    adj_list: Vec<Vec<usize>>,
    vertices: usize,
}

impl Graph {
    // Initialize a new graph with a given number of vertices
    fn new(vertices: usize) -> Self {
        let adj_list = vec![Vec::new(); vertices];
        Graph { adj_list, vertices }
    }

    // Add an edge between two vertices
    fn add_edge(&mut self, src: usize, des: usize) {
        self.adj_list[src].push(des);
        self.adj_list[des].push(src); // Because it's an undirected graph
    }

    fn degree(&self) -> Vec<usize> {
        self.adj_list
            .iter()
            .map(|neighbors| neighbors.len())
            .collect()
    }

    fn dijkstra(&self, src: usize) -> Vec<usize> {
        let mut distances = vec![usize::MAX; self.vertices];
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        distances[src] = 0;
        queue.push_back(src);
        visited.insert(src);

        while let Some(u) = queue.pop_front() {
            for &v in &self.adj_list[u] {
                if !visited.contains(&v) {
                    visited.insert(v);
                    queue.push_back(v);
                    distances[v] = distances[u] + 1; // Since each edge cost is considered 1
                }
            }
        }

        distances
    }

    fn distances(&self) -> Vec<Vec<usize>> {
        (0..self.vertices).map(|src| self.dijkstra(src)).collect()
    }
}

fn main() {
    let mut graph = Graph::new(5); // Create a graph with 5 vertices

    graph.add_edge(0, 1);
    graph.add_edge(0, 4);
    graph.add_edge(1, 2);
    graph.add_edge(1, 3);
    graph.add_edge(1, 4);
    graph.add_edge(2, 3);
    graph.add_edge(3, 4);

    // Calculate and print degrees
    let degrees = graph.degree();
    for (node, degree) in degrees.iter().enumerate() {
        println!("Node {}: Degree {}", node, degree);
    }

    let dist_mat = graph.distances();
}
