use std::collections::{HashSet, VecDeque};

struct Graph {
    adj_list: Vec<Vec<usize>>,
    vertices: usize,
}

fn dijkstra(graph: &Graph, src: usize) -> Vec<usize> {
    let mut distances = vec![usize::MAX; graph.vertices];
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    distances[src] = 0;
    queue.push_back(src);
    visited.insert(src);

    while let Some(u) = queue.pop_front() {
        for &v in &graph.adj_list[u] {
            if !visited.contains(&v) {
                visited.insert(v);
                queue.push_back(v);
                distances[v] = distances[u] + 1; // Since each edge cost is considered 1
            }
        }
    }

    distances
}

fn single_source_shortest_path(
    graph: &Graph,
    s: usize,
) -> (Vec<usize>, Vec<Vec<usize>>, Vec<usize>, Vec<usize>) {
    let mut distances = vec![usize::MAX; graph.adj_list.len()];
    let mut shortest_paths = vec![0; graph.adj_list.len()];
    let mut predecessors: Vec<Vec<usize>> = vec![Vec::new(); graph.adj_list.len()];

    let mut queue = VecDeque::new();
    let mut stack = Vec::new();

    distances[s] = 0;
    shortest_paths[s] = 1;
    queue.push_back(s);

    while let Some(v) = queue.pop_front() {
        stack.push(v);
        for &w in &graph.adj_list[v] {
            // Path discovery
            if distances[w] == usize::MAX {
                queue.push_back(w);
                distances[w] = distances[v] + 1;
            }
            // Path counting
            if distances[w] == distances[v] + 1 {
                shortest_paths[w] += shortest_paths[v];
                predecessors[w].push(v);
            }
        }
    }

    (stack, predecessors, shortest_paths, distances)
}

fn row_sum_inv(matrix: &Vec<Vec<usize>>) -> Vec<f64> {
    matrix
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter_map(|(j, &value)| {
                    if i != j && value != 0 {
                        Some(1.0 / value as f64)
                    } else {
                        None
                    }
                })
                .sum()
        })
        .collect()
}

// Constructors
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

    // Graph from an existing adjacency list
    fn from_adj_list(adj_list: Vec<Vec<usize>>) -> Self {
        let vertices = adj_list.len();
        let mut graph = Graph::new(vertices);

        for (src, neighbors) in adj_list.into_iter().enumerate() {
            for dest in neighbors {
                graph.add_edge(src, dest);
            }
        }

        graph
    }
}

// centrality and shortest path
impl Graph {
    fn degree(&self) -> Vec<usize> {
        self.adj_list
            .iter()
            .map(|neighbors| neighbors.len())
            .collect()
    }

    fn distances(&self) -> Vec<Vec<usize>> {
        (0..self.vertices).map(|src| dijkstra(&self, src)).collect()
    }

    fn closeness_centrality(&self) -> Vec<f64> {
        let d = self.distances();
        row_sum_inv(&d)
    }

    fn betweenness_centrality(&self) -> Vec<f64> {
        let mut centrality = vec![0.0; self.adj_list.len()];
        for s in 0..self.adj_list.len() {
            let (mut stack, predecessors, shortest_paths, distances) =
                single_source_shortest_path(self, s);

            let mut dependency = vec![0.0; self.adj_list.len()];
            while let Some(w) = stack.pop() {
                for &v in &predecessors[w] {
                    let coeff = (shortest_paths[v] as f64 / shortest_paths[w] as f64)
                        * (1.0 + dependency[w]);
                    dependency[v] += coeff;
                }
                if w != s {
                    centrality[w] += dependency[w];
                }
            }
        }

        // Normalization step (optional, depending on the application)
        // let norm = 1.0 / ((self.adj_list.len() - 1) * (self.adj_list.len() - 2)) as f64;
        for value in centrality.iter_mut() {
            *value *= 1.0 / 2.0;
        }

        centrality
    }
}

fn main() {
    let mut graph = Graph::new(5);

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
    for row in &dist_mat {
        for &elem in row {
            print!("{} ", elem);
        }
        println!();
    }
    println!("Closeness");
    let cc = graph.closeness_centrality();
    println!("{:?}", cc);

    println!("Betweenness");
    let bc = graph.betweenness_centrality();
    println!("{:?}", bc);
}
