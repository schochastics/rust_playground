extern crate nalgebra as na;
extern crate nalgebra_sparse as na_sparse;
use crate::graph::Graph;
use na::DVector;
use na_sparse::csr::CsrMatrix;
use std::collections::VecDeque;

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

fn distance_matrix(graph: &Graph) -> Vec<Vec<usize>> {
    let mut matrix = vec![vec![usize::MAX; graph.adj_list.len()]; graph.adj_list.len()];

    for s in 0..graph.adj_list.len() {
        let (_, _, _, distances) = single_source_shortest_path(graph, s);
        matrix[s] = distances;
    }

    matrix
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

fn power_iteration(matrix: &CsrMatrix<f64>, max_iters: usize, tolerance: f64) -> DVector<f64> {
    let n = matrix.nrows();
    let mut b_k = DVector::from_element(n, 1.0); // Initial guess
    let mut b_k1: DVector<f64>;

    for _ in 0..max_iters {
        // Multiply matrix by vector
        b_k1 = matrix * &b_k;

        // Normalize the resulting vector
        let norm = b_k1.norm();
        b_k1 /= norm;

        // Check for convergence
        if (&b_k1 - &b_k).norm() < tolerance {
            break;
        }
        b_k = b_k1;
    }

    b_k
}

impl Graph {
    pub fn degree(&self) -> Vec<usize> {
        self.adj_list
            .iter()
            .map(|neighbors| neighbors.len())
            .collect()
    }

    pub fn closeness_centrality(&self) -> Vec<f64> {
        let d = distance_matrix(self);
        row_sum_inv(&d)
    }

    pub fn betweenness_centrality(&self) -> Vec<f64> {
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

    pub fn eigenvector_centrality(&self) -> DVector<f64> {
        let adj_mat = &self.to_adjacency_matrix_sparse();
        let eigenvector = power_iteration(adj_mat, 1000, 1e-10);
        eigenvector
    }
}
