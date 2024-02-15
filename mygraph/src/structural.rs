use crate::graph::Graph;
extern crate rayon;
use rayon::prelude::*;

impl Graph {
    pub fn count_triangles(&self) -> usize {
        // Ensure each adjacency list is sorted to allow binary search
        let sorted_adj_list: Vec<Vec<usize>> = self
            .adj_list
            .par_iter()
            .map(|neighbors| {
                let mut sorted_neighbors = neighbors.clone();
                sorted_neighbors.sort_unstable();
                sorted_neighbors
            })
            .collect();

        // Use a parallel iterator to iterate over vertices
        let triangle_counts: Vec<usize> = (0..self.adj_list.len())
            .into_par_iter()
            .map(|u| {
                let mut local_count = 0;
                for &v in &sorted_adj_list[u] {
                    if v > u {
                        // Ensure we only consider each triangle once
                        for &w in &sorted_adj_list[u] {
                            if w > v && sorted_adj_list[v].binary_search(&w).is_ok() {
                                local_count += 1;
                            }
                        }
                    }
                }
                local_count
            })
            .collect();

        // Sum up the local counts and divide by 3, as each triangle is counted three times
        triangle_counts.into_iter().sum::<usize>() / 3
    }
}
