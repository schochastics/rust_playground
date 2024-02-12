extern crate nalgebra as na;
extern crate nalgebra_sparse as na_sparse;
use crate::graph::Graph;
use na::DVector;
use na_sparse::csr::CsrMatrix;
use std::collections::VecDeque;

extern crate rayon;
use rayon::prelude::*;
use std::sync::Mutex;

fn single_source_shortest_path(
    graph: &Graph,
    s: usize,
) -> (Vec<usize>, Vec<Vec<usize>>, Vec<usize>, Vec<usize>) {
    let mut distances = vec![usize::MAX; graph.vertices];
    let mut shortest_paths = vec![0; graph.vertices];
    let mut predecessors: Vec<Vec<usize>> = vec![Vec::new(); graph.vertices];

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
    let mut matrix = vec![vec![usize::MAX; graph.vertices]; graph.vertices];

    for s in 0..graph.vertices {
        let (_, _, _, distances) = single_source_shortest_path(graph, s);
        matrix[s] = distances;
    }

    matrix
}

fn bfs_shortest_paths(graph: &Graph, start: usize) -> Vec<usize> {
    let mut distances = vec![usize::MAX; graph.vertices];
    let mut queue = VecDeque::new();

    distances[start] = 0;
    queue.push_back(start);

    while let Some(node) = queue.pop_front() {
        for &neighbor in &graph.adj_list[node] {
            if distances[neighbor] == usize::MAX {
                distances[neighbor] = distances[node] + 1;
                queue.push_back(neighbor);
            }
        }
    }

    distances
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
        (0..self.vertices)
            .into_par_iter()
            .map(|node| {
                let distances = bfs_shortest_paths(self, node);
                let total_distance: usize = distances.iter().filter(|&&d| d != usize::MAX).sum();

                if total_distance > 0 {
                    // Note: Adjust the formula if your graph is directed or if you want to consider different normalization
                    (self.vertices - 1) as f64 / total_distance as f64
                } else {
                    0.0
                }
            })
            .collect()
    }

    pub fn betweenness_centrality(&self) -> Vec<f64> {
        let vertices = self.vertices;
        let centrality_global = Mutex::new(vec![0.0; vertices]);

        (0..vertices).into_par_iter().for_each(|s| {
            let (mut stack, predecessors, shortest_paths, _distances) =
                single_source_shortest_path(self, s);

            let mut dependency = vec![0.0; vertices];
            let mut centrality_local = vec![0.0; vertices];
            while let Some(w) = stack.pop() {
                for &v in &predecessors[w] {
                    let coeff = (shortest_paths[v] as f64 / shortest_paths[w] as f64)
                        * (1.0 + dependency[w]);
                    dependency[v] += coeff;
                }
                if w != s {
                    centrality_local[w] += dependency[w];
                }
            }

            let mut centrality = centrality_global.lock().unwrap();
            for i in 0..vertices {
                centrality[i] += centrality_local[i];
            }
        });

        // Normalization step
        let mut centrality = centrality_global.into_inner().unwrap();
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
