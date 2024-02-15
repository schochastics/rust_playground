use crate::graph::Graph;

impl Graph {
    // Assuming an additional function to build a community to nodes mapping
    fn community_to_nodes(&self, communities: &Vec<usize>) -> Vec<Vec<usize>> {
        let mut community_map: Vec<Vec<usize>> =
            vec![vec![]; *communities.iter().max().unwrap() as usize + 1];
        for (node, &community) in communities.iter().enumerate() {
            community_map[community].push(node);
        }
        community_map
    }

    pub fn calculate_modularity(&self, communities: &Vec<usize>) -> f64 {
        let m = self.total_degree() as f64 / 2.0;
        let community_map = self.community_to_nodes(communities);

        let mut total_modularity = 0.0;

        for (community_id, nodes) in community_map.iter().enumerate() {
            let mut internal_edges = 0;
            let mut total_degree = 0;

            // Calculate total degree of nodes in the community and internal edges
            for &node in nodes {
                total_degree += self.degree(node);
                internal_edges += self.adj_list[node]
                    .iter()
                    .filter(|&&neighbor| communities[neighbor] == community_id)
                    .count();
            }

            let expected_edges = total_degree as f64 * total_degree as f64 / (2.0 * m);
            total_modularity +=
                internal_edges as f64 / (2.0 * m) - expected_edges / (2.0 * m * 2.0);
        }

        total_modularity
    }
}
