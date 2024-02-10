pub struct Graph {
    pub adj_list: Vec<Vec<usize>>,
    pub vertices: usize,
}

// Constructors
impl Graph {
    // Initialize a new graph with a given number of vertices
    pub fn new(vertices: usize) -> Self {
        let adj_list = vec![Vec::new(); vertices];
        Graph { adj_list, vertices }
    }

    // Add an edge between two vertices
    pub fn add_edge(&mut self, src: usize, des: usize) {
        self.adj_list[src].push(des);
        self.adj_list[des].push(src); // Because it's an undirected graph
    }

    // Graph from an existing adjacency list
    pub fn from_adj_list(adj_list: Vec<Vec<usize>>) -> Self {
        let vertices = adj_list.len();
        let mut graph = Graph::new(vertices);

        for (src, neighbors) in adj_list.into_iter().enumerate() {
            for dest in neighbors {
                graph.add_edge(src, dest);
            }
        }

        graph
    }

    // Graph from an existing edgelist
    pub fn from_edgelist(edge_list: Vec<(usize, usize)>, vertices: usize) -> Self {
        let mut graph = Graph::new(vertices);

        for (src, dest) in edge_list {
            graph.add_edge(src, dest);
        }

        graph
    }
}
