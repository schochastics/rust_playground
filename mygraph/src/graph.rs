extern crate nalgebra as na;
extern crate nalgebra_sparse as na_sparse;
use na_sparse::csr::CsrMatrix;

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
    pub fn from_adjlist(adj_list: Vec<Vec<usize>>) -> Self {
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

    // Convert the adjacency list to an adjacency matrix
    pub fn to_adjacency_matrix(&self) -> Vec<Vec<i32>> {
        let mut matrix = vec![vec![0; self.vertices]; self.vertices];

        for (node, edges) in self.adj_list.iter().enumerate() {
            for &edge in edges {
                matrix[node][edge] = 1;
                matrix[edge][node] = 1; // undirected
            }
        }

        matrix
    }

    pub fn to_adjacency_matrix_sparse(&self) -> CsrMatrix<f64> {
        let mut row_indices: Vec<usize> = Vec::new();
        let mut col_indices: Vec<usize> = Vec::new();
        let mut values: Vec<f64> = Vec::new();

        for (node, edges) in self.adj_list.iter().enumerate() {
            for &edge in edges {
                row_indices.push(node);
                col_indices.push(edge);
                values.push(1.0); // Edge weight, assuming 1.0 for unweighted graphs
                                  // For undirected graphs, add the symmetric entry as well
                row_indices.push(edge);
                col_indices.push(node);
                values.push(1.0);
            }
        }

        // Create a CSR matrix
        CsrMatrix::try_from_csr_data(
            self.vertices,
            self.vertices,
            row_indices,
            col_indices,
            values,
        )
        .expect("Failed to create CSR matrix")
    }
}
