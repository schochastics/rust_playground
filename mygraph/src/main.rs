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
    fn add_edge(&mut self, v1: usize, v2: usize) {
        self.adj_list[v1].push(v2);
        self.adj_list[v2].push(v1); // Because it's an undirected graph
    }

    // Print the adjacency list (for debugging purposes)
    fn print(&self) {
        for (i, neighbors) in self.adj_list.iter().enumerate() {
            println!("Vertex {}: {:?}", i, neighbors);
        }
    }

    fn degree(&self) -> Vec<usize> {
        self.adj_list
            .iter()
            .map(|neighbors| neighbors.len())
            .collect()
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

    graph.print(); // Print the adjacency list to see the graph structure

    // Calculate and print degrees
    let degrees = graph.degree();
    for (node, degree) in degrees.iter().enumerate() {
        println!("Node {}: Degree {}", node, degree);
    }
}
